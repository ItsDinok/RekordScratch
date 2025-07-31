#![allow(non_snake_case)]
use lofty::{read_from_path, ItemKey, TaggedFileExt};
use indicatif::{ProgressBar, ProgressStyle};
use sysinfo::{System, SystemExt, DiskExt};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::path::{Path};
use walkdir::WalkDir;
use std::fs::File;
use clap::Parser;
use std::fs;
use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use dirs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod app;
mod UIManager;

use app::App;
use UIManager::ui;

//-------------------------------------------------------------------------------------------------------------------------------------
// TASKLIST
//
// TODO: Predictive time analysis on progress bar
// TODO: Break up main function
// TODO: Clean renderer
// TODO: Smart error messages and error clearing
// TODO: Untangle main()
// TODO: Playlist view

//--------------------------------------------------------------------------------------------------------------------------------------
// REGION: Path detection

// This function detects external drives and checks if they are rekordboxed
// RETURNS: Drive letter
fn DetectRemovableDrives() -> String {
    let mut sys = System::new_all();
    sys.refresh_disks_list();

    // Iterate through all disks
    for disk in sys.disks() {
        if disk.is_removable() {
            // Check for rekordboxing 
            let mountPoint = disk.mount_point().to_string_lossy();
            if let Some(driveLetter) = mountPoint.chars().next() {
                let driveLetterString = driveLetter.to_string();
                if DetectRekordboxMarkers(&driveLetterString) {
                    return driveLetterString;
                }
            }
        }
    }
    String::new()
}

// This function checks if a USB has the standard rekordbox stuff
// RETURNS: Boolean corresponding to if it is a rekordbox USB
fn DetectRekordboxMarkers(driveLetter: &str) -> bool {
    // Assemble path
    let path = format!("{}:\\", driveLetter); 

    // Rekordbox sticks have a "Contents" and a "PIONEER" folder
    let isContents = Path::new(&path).join("Contents").is_dir();
    let isPioneer = Path::new(&path).join("PIONEER").is_dir();

    isContents && isPioneer
}

// This detects the users' desktop
// RETURNS: String corresponding to users desktop
fn GetDesktopPath() -> String {
    let deskPath = dirs::desktop_dir().and_then(|path| path.to_str().map(|s| s.to_string()));

    let deskPath = match deskPath {
        Some(path) => path,
        None => {
            eprintln!("Could not detect desktop directory");
            std::process::exit(1);
        }
    };

    return deskPath;
}

// ENDREGION
// -------------------------------------------------------------------------------------------------------------------------------------


// -------------------------------------------------------------------------------------------------------------------------------------
// REGION: TrackMap construction

// This parses all playlist.txt files in a directory and adds titles to trackMap
// RETURNS: Nothing, but modifies the trackMap
fn BuildMapFromTxt(trackMap: &mut HashMap<String, String>, txtPath: &str) -> std::io::Result<()> {
    // Iterate through all txt files in the directory
    for entryResult in fs::read_dir(txtPath)? {
        let entry = entryResult?;
        let path = entry.path();

        // Check if it is a file and ends in .txt
        if path.is_file() {
            if let Some(ext) = path.extension() {
                // run parser function
                if ext == "txt" {
                    ExtractTitlesFromFile(&path, trackMap)?;
                }
            }
        }
    }

    Ok(())
}

// This populates the hashmap with the titles from the provided txt
// RETURNS: Error, but main contribution is changing the trackMap
fn ExtractTitlesFromFile(filepath: &Path, map: &mut HashMap<String, String>) -> std::io::Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let filename = filepath.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Iterate throigh each entry in file
    for (i, line_res) in reader.lines().enumerate() {
        let line = line_res?;

        // Skip header
        if i == 0 {continue;} 
   
        // Third column is track title
        // Track title ALWAYS exists
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() > 1 {
            let title = parts[2].trim().to_string();

               map.insert(title, filename.clone());
        }
    }

    Ok(())
}

// ENDREGION
// -------------------------------------------------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------------------------------------------------
// REGION: Copy files

// This gets the title from track metadata
// RETURNS: String or error
fn ExtractTitleFromPath(path: &Path) -> anyhow::Result<Option<String>> {
    let taggedFile = read_from_path(path)?;

    if let Some(tag) = taggedFile.primary_tag() {
        if let Some(title) = tag.get_string(&ItemKey::TrackTitle) {
            return Ok(Some(title.to_string()));
        }
    }
    Ok(None)
}

fn ExtractGenreFromPath(path: &Path) -> anyhow::Result<Option<String>> {
    let taggedFile = read_from_path(path)?;
    
    if let Some(tag) = taggedFile.primary_tag() {
        if let Some(genre) = tag.get_string(&ItemKey::Genre) {
            return Ok(Some(genre.to_string()));
        }
    }

    // Default to unknown genre
    // This is to ensure ALL files get moved, organised or not
    Ok(Some("Unknown Genre".to_string()))
}

// This function creates the RekordCrates folder on the desktop, reducing user input
// RETURNS: Nothing, it modifies OS state
fn CreatePlaylistsFolder(deskPath: &str) {
    let folder = Path::new(deskPath).join("RekordCrates");
    fs::create_dir_all(folder).expect("Unable to create playlists folder.");
}

// This copies the track to folders
// RETURNS: None this is one of the terminal functions
fn CopyTrackToFolder(outputRoot: &Path, folderName: &str, srcPath: &Path) -> std::io::Result<()> {
    // Clean folder name by removing .txt
    let folder = folderName.replace(".txt", "");
    
    // Build destination directory and create it
    let destDir = outputRoot.join(&folder);
    fs::create_dir_all(&destDir)?;

    // Build destination file path
    let filename = srcPath.file_name().unwrap();
    let mut destPath = destDir;
    destPath.push(filename);

    // Copy file
    fs::copy(srcPath, &destPath)?;

    Ok(())
}

// This function creates a RekordCrates subfolder on the desktop, corresponding to genre (Fallback)
// RETURNS: Nothing, modifies OS state
fn Genre_CopyTrackToFolder(outputRoot: &Path, folderName: &str, srcPath: &Path) -> std::io::Result<()> {
    // Clean folder name by removing .txt
    let folder = folderName.replace(".txt", "");
    let destDir = outputRoot.join("Unsorted").join(&folder);

    // Create directory structure
    fs::create_dir_all(&destDir)?;

    // Build destination filepath
    let filename = srcPath.file_name().unwrap();
    let destPath = destDir.join(filename);

    // Copy file
    fs::copy(srcPath, &destPath)?;

    Ok(())
}

// This copies the files to their respective folders
// RETURNS: Nothing, this is the final function
fn MoveAllMp3(trackMap: &HashMap<String, String>, root: &str, deskPath: &str) {
    // UX Debug information
    let mut tracksNotMatched = 0;
    let mut tracksMatched = 0;
    let mut unsorted = Vec::<String>::new();
    CreatePlaylistsFolder(deskPath);

    // Collect all MP3 files into a vec
    let entries: Vec<_> = WalkDir::new(root).into_iter().filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext.eq_ignore_ascii_case("mp3"))).collect();

    // Set up progress bar
    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {pos}/{len}\n{msg}").unwrap());

    // Iterate through all .mp3 files in Contents
    for entry in entries {
        let mut matched = false;
        
        let path = entry.path();
        let outputRoot = Path::new(deskPath).join("RekordCrates");

        // Extract title and compare against dictionary
        if let Ok(Some(title)) = ExtractTitleFromPath(path) {
            bar.set_message(format!("Processing: {}", title));

            if let Some(folder) = trackMap.get(&title) {
                if let Err(e) = CopyTrackToFolder(&outputRoot, folder, path) {
                    eprintln!("Failed to copy {}: {}", path.display(), e);
                }
                else {
                    tracksMatched += 1;
                    matched = true;
                }
            }
        }
        // Search by filename instead (sometimes the way)
        else if let Some(stem) = path.file_stem().and_then(|s| s.to_str()){
            if let Some(folder) = trackMap.get(stem) {
                if let Err(e) = CopyTrackToFolder(&outputRoot, folder, path) {
                    eprintln!("Failed to copy {}: {}", path.display(), e);
                }
                else {
                    tracksMatched += 1;
                    matched = true;
                }
            }
        }

        if !matched {
            // Default to genre data
            if let Ok(Some(genre)) = ExtractGenreFromPath(path) {
                if let Err(e) = Genre_CopyTrackToFolder(&outputRoot, &genre, path) {
                    eprintln!("Failed to copy {}: {}", path.display(), e);
                }
            }

            // No match found in dictionary
            tracksNotMatched += 1;
            let trackTitle = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown filename");
            unsorted.push(trackTitle.to_string());
        }

        bar.inc(1);
   }

    println!("{} tracks not matched.", tracksNotMatched);
    println!("{} tracks matched successfully.", tracksMatched);

    let mut file = File::create("NotMatched.txt").expect("Error creating output file");
    for line in unsorted {
        writeln!(file, "{}", line).expect("Failed to write to file.");
    }
}

// ENDREGION
// --------------------------------------------------------------------------------------------------------------------------------------

// --------------------------------------------------------------------------------------------------------------------------------------
// REGION: Argument/flag manager

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Playlists.txt path (-t or --target)
    #[arg(short = 't', long = "target")]
    target: Option<String>,
}

// This sets the location of the playlists.txt files
// RETURNS: Nothing, file will terminate here if not valid
fn SetTxtFileLocation() -> String {
    let path = Path::new("Playlists");
    if path.exists() && path.is_dir() {
        return path.to_str().unwrap().to_string();
    }
    else {
        return String::new();
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    // Flags
    let args = Args::parse();

    // Program state
    let trackMap = Arc::new(Mutex::new(HashMap::new()));

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = Arc::new(Mutex::new(App::new()));

    let appClone = Arc::clone(&app);
    std::thread::spawn(move || {
        // Long-running work
        let mut app = appClone.lock().unwrap();
        app.SetStatusMessage("Copying files...");
        drop(app); // Release lock before slow work

    });

    // Check for paths and drives
    let letter = DetectRemovableDrives();
    if !letter.is_empty() {
        if let Ok(mut app) = app.lock() {
            app.SetDriveLetter(format!("{}:\\", letter));
            app.SetDriveStatus(true);
        }
    }

    // User having no desktop is an unrecoverable issue
    let desktopPath = GetDesktopPath();
    // Lock mutex and access
    if let Ok(mut app) = app.lock() {
        app.SetDesktopStatus(true);
    }
   
    // Check for playlists
    let mut isPlaylistDetected = false;
    
    let txtPath = args.target.unwrap_or_else(|| SetTxtFileLocation());
    if !txtPath.is_empty() {
        if let Ok(mut app) = app.lock() {
            app.SetPlaylistStatus(true);
            isPlaylistDetected = true;
        }
    }

    loop {
        let appGuard = app.lock().unwrap();
        terminal.draw(|f| ui(f, &appGuard))?;
        drop(appGuard);

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Exit
                    KeyCode::Char('q') => break,
                
                    // Rescan drive (s for scan)
                    KeyCode::Char('s') => {
                        let letter = DetectRemovableDrives();
                        let mut app = app.lock().unwrap();
                        if letter.is_empty() {
                            app.SetError("No drive detected.");
                            app.SetDriveLetter("N/A");
                            app.SetDriveStatus(false);
                        } else {
                            app.SetDriveLetter(format!("{}:\\", letter));
                            app.SetDriveStatus(true);
                            app.SetStatusMessage("Drive detected.");
                        }
                    },
                
                    // Main logic, r for run
                    KeyCode::Char('r') => {
                        let appClone = Arc::clone(&app);
                        std::thread::spawn (move || {
                            
                        });
                    },

                    _ => continue
                }
            }
           
            let shouldBuild = {
                let app = app.lock().unwrap();
                app.playlist_detected && app.track_map_created == false
            };

            // Populate trackmap when needed
            if shouldBuild {
                let appClone = Arc::clone(&app);
                let txtPathClone = txtPath.clone();
                let trackMapClone = Arc::clone(&trackMap);

                std::thread::spawn(move || {
                    {
                        let mut app = appClone.lock().unwrap();
                        app.SetStatusMessage("Building trackmap");
                    }
                    {
                        let mut map = trackMapClone.lock().unwrap();
                        BuildMapFromTxt(&mut map, &txtPathClone);
                    }
                    {
                        let mut app = appClone.lock().unwrap();
                        app.SetStatusMessage("Trackmap built");
                        app.track_map_created = true;
                    }
                });
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

   
    // TODO: Make this step dynamic (or local?)
    //BuildMapFromTxt(&mut trackMap, &txtPath)?;
    let originPath = "";
    //MoveAllMp3(&trackMap, &originPath, &desktopPath);

    Ok(())
}

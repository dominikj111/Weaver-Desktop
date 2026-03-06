# Weaver Desktop Architecture Roadmap

This document outlines the architectural components for Weaver Desktop. The focus is on **functionality first** - get the current template working, then iterate.

---

## Development Philosophy

> **Start simple. Ship first. Abstract later.**

1. Complete the current desktop template as MVP
2. Add widgets to the working template
3. Abstract layout/templating when needed for second template
4. Polish (animations, transitions) comes last

---

## Current State ✅

| Component | Status | Location |
|-----------|--------|----------|
| Theming system | ✅ Complete | `crates/weaver_lib/src/theme/` |
| Event/Command bus | ✅ Complete | `crates/weaver_lib/src/commands/` |
| Icon system | ✅ Complete | `crates/weaver_lib/src/icons/` |
| Shell components | ✅ Complete | `crates/weaver_desktop_shell/src/components/` |
| Reactive primitives | ✅ Complete | `crates/weaver_lib/src/reactive/` |
| Widget system (Flexbox) | ✅ Complete | `crates/weaver_desktop_shell/src/components/widget.rs` |
| ImageSurface | ✅ Complete | `crates/weaver_desktop_shell/src/components/image_surface.rs` |
| DesktopShell (widget-based) | ✅ Complete | `crates/weaver_desktop_shell/src/components/desktop_shell.rs` |
| Desktop widgets | ✅ Complete | `IconGridWidget`, `DesktopImageWidget` in desktop_shell.rs |

---

## Phase 1: Complete Current Template (MVP) 🎯

**Goal:** Get the desktop template fully functional and shippable.

### What's needed (from TODO.md)

- [ ] **View System & Navigation**
  - [ ] View state management (Dashboard, Hardware, Profiles, System, Apps, Files, Settings)
  - [ ] Current view/path indicator (breadcrumb)
  
- [ ] **Dashboard View** (default home)
  - [ ] System status (CPU, RAM, Disk bars)
  - [ ] Active services status
  - [ ] Quick actions (Update, Backup, Reboot)
  - [ ] Profile information display
  
- [ ] **Hardware Control View** (unique differentiator!)
  - [ ] GPIO pin states and toggle controls
  - [ ] PWM channel sliders
  - [ ] MCU connection status
  
- [ ] **Application Launcher**
  - [ ] Launch external apps fullscreen
  - [ ] Hide Weaver Desktop while app runs
  - [ ] Auto-return when app exits
  - [ ] Hardcoded common apps (Firefox, Kodi, Terminal)
  
- [ ] **Essential Desktop Components** (from DESKTOP_COMPONENTS.md)
  - [ ] App Launcher grid
  - [ ] Settings Panel container
  - [ ] Quick Settings dropdown
  - [ ] Power Menu (shutdown, reboot, suspend)
  
- [ ] **System Integration**
  - [ ] workmeshd communication (Target abstraction)
  - [ ] Package management integration
  - [ ] Service control

### Current template features (done)

- ✅ Background image
- ✅ Top bar (solid, rounded)
- ✅ Bottom bar (transparent overlay)
- ✅ Menu button
- ✅ Toast notifications
- ✅ Command bus
- ✅ Task spawner
- ✅ Terminal (alacritty backend)
- ✅ Log panel

### Keyboard Navigation (from DESKTOP_COMPONENTS.md)

Visual hint system when Super/Meta key is pressed:

- Press Super → see hints on all interactive elements → press key → action
- Zero learning curve, hints always visible when needed
- Touch-friendly (hints can be tapped)

---

## Phase 2: Widget System

**Goal:** Add pluggable widgets to the working desktop template.

> **⚠️ Pre-work: Evaluate layout engine before deepening flexbox implementation**
>
> The current `widget.rs` has a hand-rolled CSS Flexbox-inspired layout engine. Before investing further, evaluate whether an existing crate covers this better:
>
> | Crate | Approach | Notes |
> |-------|----------|-------|
> | [`taffy`](https://github.com/DioxusLabs/taffy) | Full Flexbox + CSS Grid engine | Best spec accuracy; compute layout → draw egui widgets in returned rects. Bevy UI uses this. |
> | [`egui_flex`](https://github.com/lucasmerlin/hello_egui/tree/main/crates/egui_flex) | Flex rows/columns natively in egui | Simpler, less spec-complete, no adapter layer needed |
> | [`egui_extras`](https://docs.rs/egui_extras) | StripBuilder, table layouts | Not flexbox, but solid structured layout primitives |
>
> **Likely best fit:** `taffy` + egui adapter (~150 lines). Pattern: UI tree → Taffy computes rects → egui widgets rendered inside those rects. Avoids reimplementing what browsers spent years getting right.
>
> **Decision needed before Phase 2 layout work:** keep hand-rolled engine, adopt taffy, or adopt egui_flex.

```rust
pub trait Widget: Send + Sync {
    /// Unique identifier for this widget type
    fn id(&self) -> &str;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Minimum size the widget can be rendered at
    fn min_size(&self) -> egui::Vec2;
    
    /// Preferred/default size
    fn preferred_size(&self) -> egui::Vec2;
    
    /// Render the widget
    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, theme: &Theme);
    
    // Optional capabilities
    fn supports_resize(&self) -> bool { true }
    fn supports_transparency(&self) -> bool { false }
    fn supports_drag(&self) -> bool { true }
}

pub struct WidgetRegistry {
    widgets: HashMap<String, Box<dyn Widget>>,
}
```

### Built-in widgets to implement

- [ ] Clock/DateTime
- [ ] System monitor (CPU, RAM, temp)
- [ ] Media player controls
- [ ] Quick settings toggles
- [ ] App launcher grid
- [ ] Notifications center
- [ ] Weather (external API)
- [ ] Calendar (expand from popup)
- [ ] GPIO status widget
- [ ] Profile status badge

### Widget surface management

```rust
pub struct WidgetSurface {
    pub id: String,
    pub widget_id: String,
    pub rect: egui::Rect,
    pub z_order: u32,
    pub visible: bool,
    pub draggable: bool,
    pub resizable: bool,
}
```

---

## Phase 3: Layout/Template Abstraction

**Goal:** Extract layout system when building second template (kiosk, media center, etc.)

This will emerge naturally when needed. Possible layouts:

| Layout | Use Case |
|--------|----------|
| Desktop | Background, icons, top/bottom bars, widgets |
| WidgetsOnly | No chrome, just floating widgets |
| Kiosk | Single full-screen widget (coffee machine, car radio) |
| MediaCenter | Large media widget + minimal controls |
| ControlPanel | Sidebar navigation + content area |

```rust
pub enum ShellLayout {
    Desktop { 
        show_top_bar: bool,
        show_bottom_bar: bool,
        show_desktop_icons: bool,
    },
    WidgetsOnly,
    Kiosk { widget_id: String },
    MediaCenter,
    ControlPanel { sidebar_position: SidebarPosition },
    Custom(LayoutConfig),
}
```

---

## Phase 4: Configuration & Persistence

**Goal:** Save and load user preferences, layouts, widget positions.

```rust
pub struct ShellConfig {
    pub active_layout: ShellLayout,
    pub theme_name: String,
    pub widget_placements: HashMap<String, Vec<WidgetPlacement>>,
    pub panel_config: PanelConfig,
    pub preferences: UserPreferences,
}
```

Storage options:

- JSON/TOML for human-readable config
- SQLite for structured data
- Platform keychain for secrets

---

## Phase 5: Input Abstraction

**Goal:** Support different input modes when targeting touch/kiosk devices.

```rust
pub enum InputMode {
    Desktop,      // Mouse + keyboard
    Touch,        // Touch-only (kiosk)
    MediaRemote,  // Limited buttons (car radio)
    Gamepad,      // Controller navigation
}
```

---

## Phase 6: UI Fabric (Socket-Driven UI)

**Goal:** Enable external processes to declare UI through sockets, rendered by Weaver.

See [UI_FABRIC_PROPOSAL.md](./UI_FABRIC_PROPOSAL.md) for full specification.

### Core Concept

External processes (local or remote, human-written or AI-generated) declare UI structure through a socket connection. Weaver renders it inside constrained containers while maintaining sole authority over rendering, interaction, and action execution.

```
External Process ──► UI Declare (JSON)
         Weaver ──► Validate & Render
           User ──► Interaction
         Weaver ──► Semantic Event (to process)
External Process ──► State Update
```

### Key Components

```rust
/// Socket listener for UI sessions
pub struct FabricListener {
    socket_path: PathBuf,
    sessions: HashMap<SessionId, FabricSession>,
}

/// Individual UI session from external process
pub struct FabricSession {
    pub id: SessionId,
    pub capabilities: CapabilitySet,
    pub container: ContainerBinding,
    pub widgets: Vec<FabricWidget>,
    pub connection: SocketConnection,
}

/// Capability set granted to session
pub struct CapabilitySet {
    pub allowed_widgets: Vec<WidgetType>,
    pub allowed_actions: Vec<ActionPattern>,
    pub allowed_containers: Vec<ContainerType>,
}

/// Container binding for session UI
pub enum ContainerBinding {
    Widget { slot: String },
    Window { title: String, size: Option<Vec2> },
    Modal { title: String, dismissable: bool },
    Sheet { edge: Edge },
    PanelSlot { slot_id: String },
}
```

### Message Protocol

| Message | Direction | Description |
|---------|-----------|-------------|
| `session.init` | Client → Weaver | Request session with capabilities |
| `session.granted` | Weaver → Client | Confirm with assigned capabilities |
| `ui.define` | Client → Weaver | Declare/update UI structure |
| `ui.update` | Client → Weaver | Partial widget update |
| `event` | Weaver → Client | User interaction event |
| `action.result` | Weaver → Client | Result of action execution |

### Widget Types (Initial)

- `button` - Action trigger
- `label` - Static text
- `status` - Dynamic data display
- `progress` - Progress bar
- `slider` - Numeric control
- `toggle` - Boolean switch
- `text_input` - Text entry
- `select` - Dropdown selection
- `group` - Layout container

### Security Model

1. **Capability-based** — Sessions receive limited widget/action sets
2. **Action routing** — Actions go to workmeshd, not direct execution
3. **Audit logging** — Every UI-triggered action logged
4. **Container isolation** — Sessions bound to template-defined slots

### Implementation Phases

1. **Foundation** — Socket listener, session management, basic widgets
2. **Core Widgets** — Full widget set, layout engine, containers
3. **Security** — Capability negotiation, action routing, audit
4. **SDK** — Python, Node.js, Rust client libraries

### Use Cases Enabled

- Cloud applications without browsers
- AI-driven interfaces (AI proposes, Weaver governs)
- Dynamic industrial dashboards
- Remote kiosk UI updates

---

## Future Polish (Not MVP)

### Animation/Transition System

- Smooth layout transitions
- Widget animations
- Will be implemented when core functionality is solid

### Hot-Reloading Layouts

- Switch between templates without restart
- Will emerge from layout abstraction

### Profile System (from PROPOSAL.md)

- Hierarchical profiles with inheritance (base → device-specific)
- Profile sharing/export/import
- State synchronization (detect drift)
- System matches profile vs. drift indicator

### Plugin Architecture

- Load custom widgets from shared libraries
- Hot-reload during development
- Widget manifest format for metadata

### System Utilities

GUI wrappers for essential system operations. All operations dispatch commands
to backend handlers (workmeshd or TaskSpawner) - GUI remains pure.

#### Disk Management

Full-featured disk utility for partitioning, formatting, and drive management.

```rust
pub struct DiskManager {
    pub drives: Vec<DriveInfo>,
    pub selected_drive: Option<DriveId>,
    pub selected_partition: Option<PartitionId>,
    pub pending_operations: Vec<DiskOperation>,  // Queue before apply
    pub operation_mode: OperationMode,
}

pub struct DriveInfo {
    pub id: DriveId,
    pub device_path: PathBuf,         // /dev/sda, /dev/nvme0n1
    pub model: String,
    pub serial: String,
    pub size: u64,
    pub drive_type: DriveType,        // SSD, HDD, USB, SD, NVMe
    pub partitions: Vec<PartitionInfo>,
    pub health: DriveHealth,          // SMART data
    pub removable: bool,
}

pub struct PartitionInfo {
    pub id: PartitionId,
    pub device_path: PathBuf,         // /dev/sda1
    pub label: Option<String>,
    pub filesystem: FilesystemType,
    pub size: u64,
    pub used: u64,
    pub mount_point: Option<PathBuf>,
    pub flags: Vec<PartitionFlag>,    // boot, esp, lvm, etc.
}

pub enum FilesystemType {
    Ext4, Ext3, Ext2,
    Btrfs, Xfs, Zfs,
    Fat32, ExFat, Ntfs,
    Swap,
    Unknown(String),
}

pub enum DiskOperation {
    CreatePartition { drive: DriveId, size: u64, fs: FilesystemType, label: String },
    DeletePartition(PartitionId),
    ResizePartition { partition: PartitionId, new_size: u64 },
    FormatPartition { partition: PartitionId, fs: FilesystemType, label: String },
    SetLabel { partition: PartitionId, label: String },
    SetFlags { partition: PartitionId, flags: Vec<PartitionFlag> },
    Mount { partition: PartitionId, mount_point: PathBuf },
    Unmount(PartitionId),
    Eject(DriveId),
    SecureErase(DriveId),             // DANGER: Wipes everything
    CheckFilesystem(PartitionId),     // fsck
    CreatePartitionTable { drive: DriveId, table_type: PartitionTableType },
}

// Commands dispatched to backend
pub enum DiskCommand {
    RefreshDrives,
    SelectDrive(DriveId),
    SelectPartition(PartitionId),
    QueueOperation(DiskOperation),
    ClearQueue,
    ApplyOperations,                  // Execute all queued operations
    CancelOperations,
    Mount { partition: PartitionId, mount_point: PathBuf },
    Unmount(PartitionId),
    Eject(DriveId),
    ShowSmartData(DriveId),
    BenchmarkDrive(DriveId),
}
```

**Features:**

- Visual drive/partition map (like GParted)
- Create, delete, resize partitions
- Format with multiple filesystem support (ext4, btrfs, xfs, fat32, exfat, ntfs)
- Mount/unmount/eject operations
- Partition labeling and flags
- SMART health data display
- Drive benchmark tool
- Secure erase option (with multiple confirmations!)
- Queue operations → preview → apply (like GParted)
- Support for LVM, RAID visualization (read-only initially)

**Third-party Backend Integration:**

- `lsblk`, `blkid` for drive enumeration
- `parted` / `libparted` for partitioning
- `mkfs.*` utilities for formatting
- `smartctl` for SMART data
- Optional: spawn `gparted` for complex operations
- Future: `udisks2` D-Bus integration

**Safety:**

- Root/sudo required for destructive operations
- Multiple confirmation dialogs for data loss
- Clear visual indicators for mounted/in-use partitions
- Operation preview before applying

#### ISO Flasher

Create bootable USB drives from ISO images.

```rust
pub struct IsoFlasher {
    pub iso_path: Option<PathBuf>,
    pub target_device: Option<DriveId>,
    pub progress: Option<FlashProgress>,
    pub verify_after_flash: bool,
}

pub struct FlashProgress {
    pub bytes_written: u64,
    pub total_bytes: u64,
    pub speed: u64,              // bytes/sec
    pub eta: Duration,
    pub phase: FlashPhase,       // Writing, Verifying
}

pub enum IsoFlasherCommand {
    SelectIso(PathBuf),
    SelectTarget(DriveId),
    StartFlash,
    CancelFlash,
    VerifyFlash,
}
```

**Features:**

- Drag-and-drop ISO selection
- Clear drive selection with size/model info
- Progress bar with ETA and speed
- Optional verification after write
- Multi-image queue (flash multiple USBs)

#### Backup Tool

Simple file/folder backup utility.

```rust
pub struct BackupTool {
    pub backup_sources: Vec<PathBuf>,
    pub backup_destination: PathBuf,
    pub schedule: Option<BackupSchedule>,
    pub compression: CompressionType,
    pub encryption: Option<EncryptionConfig>,
    pub backup_history: Vec<BackupRecord>,
}

pub enum BackupCommand {
    AddSource(PathBuf),
    RemoveSource(PathBuf),
    SetDestination(PathBuf),
    CreateBackup,
    RestoreBackup { backup_id: BackupId, target: PathBuf },
    DeleteBackup(BackupId),
    ScheduleBackup(BackupSchedule),
    VerifyBackup(BackupId),
}
```

**Features:**

- Select folders/files to back up
- Choose destination (local, external drive, network)
- Compression options (none, gzip, zstd)
- Optional encryption
- Incremental backups
- Backup history with restore points
- Scheduled backups (daily, weekly)
- Backend: `tar`, `rsync`, or `restic`

#### System Cleanup

Free disk space by clearing caches and temporary files.

```rust
pub struct SystemCleanup {
    pub scan_results: CleanupScanResult,
    pub selected_items: HashSet<CleanupItem>,
}

pub struct CleanupScanResult {
    pub package_cache: u64,       // apt/dnf cache
    pub temp_files: u64,          // /tmp, /var/tmp
    pub user_cache: u64,          // ~/.cache
    pub log_files: u64,           // Old logs
    pub thumbnail_cache: u64,     // Image thumbnails
    pub trash: u64,               // Trash bin
    pub orphan_packages: Vec<String>,
    pub old_kernels: Vec<String>,
}

pub enum CleanupCommand {
    Scan,
    SelectItem(CleanupItem),
    DeselectItem(CleanupItem),
    SelectAll,
    DeselectAll,
    Clean,
}
```

**Features:**

- Scan for cleanable items
- Category breakdown (cache, logs, temp, trash)
- Safe defaults (won't break system)
- Preview before cleaning
- Backend: `apt clean`, `journalctl --vacuum`, `rm -rf`

### Input & Accessibility Components

#### Virtual Keyboard (Touch-First)

On-screen keyboard essential for touchscreen devices, kiosks, and tablets.

```rust
pub struct VirtualKeyboard {
    pub layout: KeyboardLayout,        // QWERTY, AZERTY, Dvorak, etc.
    pub mode: KeyboardMode,            // Standard, Numeric, Symbols, Emoji
    pub size: KeyboardSize,            // Compact, Standard, Large
    pub position: KeyboardPosition,    // Bottom, Floating
    pub theme: KeyboardTheme,          // Match system theme
    pub haptic_feedback: bool,         // Vibration on key press (if available)
    pub sound_feedback: bool,          // Click sounds
    pub auto_correct: bool,            // Basic autocorrect
    pub word_prediction: bool,         // Suggest words
}

pub enum KeyboardMode {
    Standard,           // Letters
    Numeric,            // Numbers only (PIN entry)
    Phone,              // Phone number pad
    Symbols,            // Special characters
    Emoji,              // Emoji picker (integrates with Emoji Map)
    Password,           // Scrambled layout for security
}

pub enum KeyboardLayout {
    Qwerty,
    Azerty,
    Qwertz,
    Dvorak,
    Colemak,
    Custom(LayoutConfig),
}

// Commands
pub enum KeyboardCommand {
    Show { mode: KeyboardMode, target_field: FieldId },
    Hide,
    SetLayout(KeyboardLayout),
    ToggleMode(KeyboardMode),
    TypeChar(char),
    Backspace,
    Enter,
    SwitchLanguage,
}
```

**Features:**

- Touch-optimized key sizing (minimum 48px targets)
- Swipe-to-type gesture support (future)
- Multi-language support with quick switch
- Long-press for accented characters
- Emoji quick-access button → opens Emoji Map
- Numeric mode for PIN/password entry
- Split keyboard option for tablets
- One-handed mode (compact, left/right aligned)
- Adaptive sizing based on screen dimensions

#### Colour Selector / Palette Tool

Colour picker with potential to evolve into full palette management.

```rust
pub struct ColourSelector {
    pub current_colour: Color32,
    pub format: ColourFormat,          // RGB, HSL, HSV, HEX, CMYK
    pub mode: SelectorMode,            // Wheel, Sliders, Palette
    pub recent_colours: Vec<Color32>,  // Last used
    pub saved_palettes: Vec<Palette>,  // User palettes
    pub eyedropper_active: bool,       // Pick from screen
}

pub struct Palette {
    pub name: String,
    pub colours: Vec<Color32>,
    pub category: PaletteCategory,     // Custom, Material, Pastel, etc.
}

pub enum SelectorMode {
    Wheel,              // HSV colour wheel
    Sliders,            // RGB/HSL sliders
    Grid,               // Preset colour grid
    Palette,            // Custom palette view
    Eyedropper,         // Pick from anywhere on screen
}

pub enum ColourFormat {
    Hex,                // #FF5733
    Rgb,                // rgb(255, 87, 51)
    Hsl,                // hsl(9, 100%, 60%)
    Hsv,                // hsv(9, 80%, 100%)
    Cmyk,               // cmyk(0, 66, 80, 0)
}

// Commands
pub enum ColourCommand {
    OpenSelector { initial: Option<Color32>, callback_field: FieldId },
    PickColour(Color32),
    SaveToPalette { colour: Color32, palette: String },
    CreatePalette(String),
    DeletePalette(String),
    ExportPalette { palette: String, format: ExportFormat },
    ImportPalette(PathBuf),
    ToggleEyedropper,
}
```

**Features:**

- HSV colour wheel with saturation/value selector
- RGB/HSL slider inputs
- Hex input field with validation
- Recent colours history (last 20)
- Custom palette creation and management
- Eyedropper tool to pick any screen colour
- Colour harmony suggestions (complementary, analogous, triadic)
- Accessibility: contrast ratio checker (WCAG)
- Copy colour in multiple formats (Hex, RGB, HSL)
- Import/export palettes (ASE, GPL, JSON)

**Palette Evolution (Future):**

- Named palettes with categories
- Palette sharing/import
- Theme colour extraction from images
- Material Design / Tailwind presets

#### Character & Emoji Map

Comprehensive character picker for special symbols, unicode, and emoji.

```rust
pub struct CharEmojiMap {
    pub mode: CharMapMode,
    pub search_query: String,
    pub selected_category: Category,
    pub recent_chars: Vec<char>,
    pub favourites: Vec<char>,
    pub skin_tone: SkinTone,          // Emoji modifier
}

pub enum CharMapMode {
    Emoji,              // 😀 🎉 🚀
    Symbols,            // © ® ™ ° ± × ÷
    Arrows,             // → ← ↑ ↓ ⇒ ⇐
    Math,               // ∑ ∫ ∞ √ π θ
    Currency,           // $ € £ ¥ ₿ ₹
    Greek,              // α β γ δ ε
    Latin,              // ñ ü ö ç
    Technical,          // ⌘ ⌥ ⇧ ⌃ (Mac keys)
    Dingbats,           // ✓ ✗ ★ ☆ ♠ ♣
    Braille,            // ⠁ ⠂ ⠃
    All,                // Full unicode browser
}

pub enum Category {
    // Emoji categories
    SmileysEmotion,
    PeopleBody,
    AnimalsNature,
    FoodDrink,
    TravelPlaces,
    Activities,
    Objects,
    Symbols,
    Flags,
    // Symbol categories
    Mathematical,
    Technical,
    Arrows,
    Currency,
    Punctuation,
    Letterlike,
}

// Commands
pub enum CharMapCommand {
    Open { mode: CharMapMode },
    Search(String),
    SelectCategory(Category),
    InsertChar(char),
    AddToFavourites(char),
    RemoveFromFavourites(char),
    CopyToClipboard(String),
    SetSkinTone(SkinTone),
}
```

**Features:**

- Categorized emoji browser with search
- Unicode character categories
- Search by name or description ("heart", "arrow right")
- Recently used section
- Favourites/starred characters
- Skin tone modifier for emoji
- Click to insert into active text field
- Click to copy to clipboard
- Unicode code point display (U+1F600)
- Font preview (how char looks in different fonts)

### Accessibility (from DESKTOP_COMPONENTS.md)

- Visual keyboard navigation (Super key hints)
- Full keyboard-only operation
- Screen reader support
- High contrast themes
- Reduced motion mode
- Virtual keyboard for touch (see above)

### Additional Recommended Components

Components that would significantly enhance the Weaver Desktop experience:

#### Network Tools

```rust
pub struct NetworkTools {
    pub interfaces: Vec<NetworkInterface>,
    pub connections: Vec<NetworkConnection>,
    pub wifi_networks: Vec<WifiNetwork>,
}

// Components
| Component         | Description                           |
|-------------------|---------------------------------------|
| Network Manager   | View/configure network interfaces     |
| WiFi Scanner      | Scan and connect to wireless networks |
| VPN Manager       | Configure VPN connections             |
| Firewall GUI      | Manage iptables/nftables rules        |
| Port Scanner      | Simple port check utility             |
| Speed Test        | Network bandwidth test                |
```

#### Process & Task Manager

```rust
pub struct ProcessManager {
    pub processes: Vec<ProcessInfo>,
    pub sort_by: ProcessSortField,
    pub filter: Option<String>,
    pub show_system_processes: bool,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_percent: f32,
    pub user: String,
    pub state: ProcessState,
}

// Commands
pub enum ProcessCommand {
    Refresh,
    Kill(Pid),
    SetPriority { pid: Pid, priority: i32 },
    SearchProcess(String),
}
```

**Features:**

- Real-time process list
- CPU/Memory usage per process
- Kill/terminate processes
- Priority adjustment (nice)
- Search/filter processes
- Sort by CPU, memory, name, PID

#### System Information

```rust
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub total_memory: u64,
    pub gpu_info: Vec<GpuInfo>,
    pub disk_info: Vec<DiskInfo>,
    pub network_interfaces: Vec<NetInterface>,
    pub uptime: Duration,
}
```

**Features:**

- Hardware overview (CPU, GPU, RAM)
- OS and kernel version
- Storage devices
- Network interfaces
- Uptime and load average
- Export to text/JSON

#### Screen Recording & Screenshot

```rust
pub struct ScreenCapture {
    pub capture_mode: CaptureMode,
    pub include_cursor: bool,
    pub output_format: ImageFormat,
    pub output_path: PathBuf,
    pub delay_seconds: u32,
}

pub enum CaptureMode {
    FullScreen,
    Window,
    Region { x: i32, y: i32, w: u32, h: u32 },
    Monitor(u32),
}

// For recording
pub struct ScreenRecorder {
    pub is_recording: bool,
    pub audio_source: Option<AudioSource>,
    pub output_format: VideoFormat,
    pub quality: RecordingQuality,
}
```

**Features:**

- Screenshot: full screen, window, region
- Delay timer
- Include/exclude cursor
- Annotation tools (arrows, boxes, text)
- Screen recording with audio
- GIF capture

#### QR Code Tools

```rust
pub struct QrCodeTool {
    pub mode: QrMode,
    pub content: String,
    pub generated_image: Option<Image>,
}

pub enum QrMode {
    Generate,   // Create QR from text/URL
    Scan,       // Scan QR from camera or screen
}
```

**Features:**

- Generate QR codes from text/URLs
- Scan QR codes via camera
- Copy QR content
- Save QR as image
- WiFi network QR generation

#### Font Manager

```rust
pub struct FontManager {
    pub installed_fonts: Vec<FontInfo>,
    pub preview_text: String,
    pub preview_size: f32,
}

pub struct FontInfo {
    pub family: String,
    pub style: String,
    pub file_path: PathBuf,
    pub is_system: bool,
}

pub enum FontCommand {
    InstallFont(PathBuf),
    RemoveFont(String),
    PreviewFont { family: String, text: String },
    ExportFontList,
}
```

**Features:**

- Browse installed fonts
- Preview text in different fonts/sizes
- Install/remove fonts
- Font information (glyphs, styles)
- Character map per font

#### Archive Manager

```rust
pub struct ArchiveManager {
    pub current_archive: Option<ArchiveInfo>,
    pub contents: Vec<ArchiveEntry>,
    pub extraction_progress: Option<Progress>,
}

pub enum ArchiveCommand {
    Open(PathBuf),
    Extract { archive: PathBuf, destination: PathBuf },
    Create { paths: Vec<PathBuf>, output: PathBuf, format: ArchiveFormat },
    AddToArchive { archive: PathBuf, paths: Vec<PathBuf> },
    Preview(ArchiveEntry),
}

pub enum ArchiveFormat {
    Zip, TarGz, TarBz2, TarXz, Tar, SevenZip, Rar,
}
```

**Features:**

- Open/browse archives
- Extract all or selected files
- Create archives (zip, tar.gz, 7z)
- Add files to existing archives
- Preview files without extracting

### Multi-Monitor Support

- Per-monitor layout configuration
- Drag widgets between monitors
- Target: Phase 2 (consumer PCs)

### Remote Target Support (from MULTI_TARGET_ARCHITECTURE.md)

- Target abstraction (LocalTarget vs RemoteTarget)
- Transparent remote terminal (no explicit SSH)
- Remote file browsing, service control
- workmeshd on each target machine

### Settings Views (from DESKTOP_COMPONENTS.md)

- WiFi, Bluetooth, Display, Audio
- Keyboard, Language/Locale, Date/Time
- Users/Accounts, Power/Battery, Storage
- Firewall, Printers, Accessibility

### Built-in Utilities (from DESKTOP_COMPONENTS.md)

- File Explorer, Image Viewer, Text Viewer
- Calculator, Notes, PDF Viewer
- Media Player (vlc-rs)

### Software Center (from DESKTOP_COMPONENTS.md)

- Unified app installation (apt, flatpak, nix, brew, appimage)
- Browse/search, install/remove, updates

---

## Target Platforms

### Phase 1: Current Focus

- Single small screens
- Kiosk displays
- TV interfaces
- Embedded systems (SBCs)

### Phase 2: Future Expansion

- Consumer PCs
- Multi-monitor setups
- Tablet/touch devices

---

## Implementation Priority

| Phase | Focus | Status |
|-------|-------|--------|
| 1 | Complete current desktop template | 🔴 In Progress |
| 2 | Widget system | ⚪ Not Started |
| 3 | Layout/template abstraction | ⚪ Not Started |
| 4 | Configuration persistence | ⚪ Not Started |
| 5 | Input mode abstraction | ⚪ Not Started |
| — | Animation & polish | ⚪ Future |

---

## Architecture Overview (Target State)

```
┌─────────────────────────────────────────────────────────────┐
│                         App                                 │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Theme     │  │  CommandBus │  │  Config/Persistence │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                     ShellManager                            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                  LayoutEngine                          │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │  │
│  │  │ Desktop  │ │  Kiosk   │ │  Media   │ │  Custom  │  │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                   SurfaceManager                            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Floating widgets, z-ordering, drag/resize handling   │  │
│  └───────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                   Widget Registry                           │
│  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐    │
│  │ Clock  │ │ Media  │ │ System │ │ Launch │ │ Custom │    │
│  └────────┘ └────────┘ └────────┘ └────────┘ └────────┘    │
└─────────────────────────────────────────────────────────────┘
```

---

## Related Documents

- [PROPOSAL.md](./PROPOSAL.md) - Project vision, core features, profile system
- [UI_FABRIC_PROPOSAL.md](./UI_FABRIC_PROPOSAL.md) - Socket-driven UI runtime specification
- [TODO.md](./TODO.md) - Detailed task list and immediate next steps
- [DESKTOP_COMPONENTS.md](./DESKTOP_COMPONENTS.md) - Component specifications, build order
- [MULTI_TARGET_ARCHITECTURE.md](./MULTI_TARGET_ARCHITECTURE.md) - Remote target support, workmeshd protocol

---

## Key Differentiators

- **Hardware Control** - GPIO, PWM, MCU integration (unique!)
- **Sub-50MB Footprint** - vs 300-600MB for traditional DEs
- **Profile-Based Reproduction** - Docker for containers, Weaver Desktop for bare metal
- **Touch-First** - Designed for 7" screens, kiosks, embedded
- **Local-First** - No cloud required, full ownership

---

*Last updated: December 2024*

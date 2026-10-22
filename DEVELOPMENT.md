# Development Guide

## Phase-by-Phase Implementation

### ✅ Phase 1: Setup Project & Basic Window (COMPLETE)

**Tujuan:** Inisialisasi project Rust dan buat basic native window

**Deliverables:**
- [x] Cargo.toml dengan semua dependencies
- [x] Struktur direktori lengkap
- [x] Basic window dengan winit
- [x] Configuration system
- [x] Logging infrastructure
- [x] Placeholder modules untuk semua features

**Files Created:**
- `src/main.rs` - Entry point dengan logging
- `src/app.rs` - Application event loop
- `src/config/` - Config management
- `src/ui/` - UI state management
- `src/platform/windows/` - Windows API placeholders
- `src/features/` - Feature module placeholders
- `src/storage/` - Database placeholders
- `src/services/` - Background service placeholders

**Testing:**
```powershell
cargo build
cargo run  # Should create a basic window (press ESC to exit)
```

---

### 🔨 Phase 2: Floating Bar & Window Management (NEXT)

**Tujuan:** Implementasi floating window native dengan always-on-top dan system tray

**Tasks:**
1. Implementasi `window_manager.rs`:
   - Set always-on-top menggunakan `SetWindowPos` dengan `HWND_TOPMOST`
   - Remove window decorations
   - Set transparent background
   - Handle multi-monitor positioning

2. Implementasi `tray.rs`:
   - Create system tray icon
   - Tray menu (Show/Hide, Settings, Exit)
   - Minimize to tray behavior

3. Window positioning:
   - Calculate position berdasarkan alignment config
   - Handle screen resolution changes
   - Save/restore window position

4. Click-through areas:
   - Implement transparent input regions
   - Only capture clicks on interactive elements

**Windows APIs yang digunakan:**
- `SetWindowPos` - Window positioning dan layering
- `SetWindowLong` / `SetWindowLongPtr` - Window styles
- `Shell_NotifyIcon` - System tray
- `EnumDisplayMonitors` - Multi-monitor support

**Testing Checklist:**
- [ ] Window appears at top center of screen
- [ ] Window stays on top of other windows
- [ ] No window decorations visible
- [ ] Background is transparent
- [ ] System tray icon appears
- [ ] Can minimize to tray and restore
- [ ] Window repositions correctly on resolution change

---

### 🎨 Phase 3: UI Framework & Animation

**Tujuan:** Setup rendering pipeline dan implementasi animasi

**Tasks:**
1. Setup wgpu rendering pipeline:
   - Initialize wgpu device dan surface
   - Create render pipeline
   - Implement basic shapes (rectangles, rounded corners)

2. Implement UI states:
   - CompactIdle: 200x40px bar
   - Hovered: Magnification effect
   - Expanded: 400x300px panel

3. Animation system:
   - Smooth transitions between states
   - Easing functions (ease-in-out)
   - Respect reduced motion preferences
   - Frame rate management

4. Basic components:
   - Button
   - Icon
   - Text label
   - Panel container

**Key Metrics:**
- Smooth 60 FPS animations
- < 5% CPU usage when idle
- < 50MB memory footprint

**Testing Checklist:**
- [ ] Compact mode renders correctly
- [ ] Hover triggers expansion animation
- [ ] Animations are smooth (no jank)
- [ ] Reduced motion mode works
- [ ] Resource usage is acceptable

---

### 💾 Phase 4: Database & Storage

**Tujuan:** Setup SQLite database dan migrations

**Tasks:**
1. Design database schema:
   ```sql
   CREATE TABLE todos (
     id INTEGER PRIMARY KEY,
     title TEXT NOT NULL,
     description TEXT,
     priority INTEGER,
     due_at DATETIME,
     completed_at DATETIME,
     created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
     updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE reminders (
     id INTEGER PRIMARY KEY,
     todo_id INTEGER,
     title TEXT NOT NULL,
     message TEXT,
     scheduled_at DATETIME NOT NULL,
     recurrence_rule TEXT,
     status TEXT NOT NULL,
     snooze_until DATETIME,
     created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
     updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
     FOREIGN KEY (todo_id) REFERENCES todos(id)
   );

   CREATE TABLE shortcuts (
     id INTEGER PRIMARY KEY,
     label TEXT NOT NULL,
     target_path TEXT NOT NULL,
     item_type TEXT NOT NULL,
     sort_order INTEGER DEFAULT 0,
     created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE settings (
     key TEXT PRIMARY KEY,
     value TEXT NOT NULL,
     updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE notification_preferences (
     category TEXT PRIMARY KEY,
     enabled INTEGER DEFAULT 1,
     hide_content INTEGER DEFAULT 0,
     updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```

2. Implement migration system
3. CRUD operations untuk setiap tabel
4. Transaction support
5. Error handling dan recovery

**Testing Checklist:**
- [ ] Database creates successfully
- [ ] Migrations run in order
- [ ] All CRUD operations work
- [ ] Foreign keys enforced
- [ ] Handles corrupted database gracefully

---

### 🚀 Phase 5: Launcher & Drag-Drop

**Tujuan:** Implementasi launcher untuk aplikasi/file dan drag-drop support

**Tasks:**
1. Shortcut management:
   - Add shortcut (file picker)
   - Remove shortcut
   - Reorder shortcuts
   - Launch target

2. Drag-drop implementation:
   - Register as drop target
   - Handle `IDataObject` dari Windows
   - Extract file paths
   - Show drop menu (Open, Save Shortcut, Add to Quick Access, Cancel)

3. File operations:
   - Validate paths
   - Get file icons
   - Open with default application
   - Handle broken shortcuts

**Windows APIs:**
- `IDropTarget` interface
- `SHGetFileInfo` - Get file icons
- `ShellExecute` - Launch files/apps

**Testing Checklist:**
- [ ] Can add shortcuts via file picker
- [ ] Can drag files from Explorer to dock
- [ ] Drop menu appears and works
- [ ] Shortcuts launch correctly
- [ ] Broken shortcuts show error message
- [ ] Icons display correctly

---

### 🎵 Phase 6: Media Integration

**Tujuan:** Integrasi Windows Global System Media Transport Controls (GSMTC)

**Tasks:**
1. Setup GSMTC listener:
   - Access `GlobalSystemMediaTransportControlsSessionManager`
   - Listen for session changes
   - Read metadata (title, artist, album, artwork)
   - Monitor playback state

2. Media controls:
   - Play/Pause
   - Next/Previous
   - Seek (if supported)

3. UI integration:
   - Display current track info
   - Show album artwork
   - Playback controls buttons
   - Progress bar

**Windows APIs:**
- `Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager`
- `Windows.Storage.Streams` - For artwork

**Testing Checklist:**
- [ ] Detects Spotify playing
- [ ] Detects Windows Media Player
- [ ] Metadata updates on track change
- [ ] Play/pause works
- [ ] Next/previous works
- [ ] Handles no active session gracefully

---

### ✅ Phase 7: To-Do List

**Tujuan:** Implementasi to-do list dengan UI dan persistence

**Tasks:**
1. Database operations:
   - Create todo
   - Read todos (all, active, completed)
   - Update todo
   - Delete todo
   - Mark complete/incomplete

2. UI components:
   - Todo list view
   - Add todo form
   - Edit todo inline
   - Filter buttons (All, Active, Completed)
   - Search/filter

3. Features:
   - Priority levels (Low, Medium, High)
   - Due dates
   - Descriptions
   - Sort by date, priority, or custom order

**Testing Checklist:**
- [ ] Can create todos
- [ ] Todos persist after restart
- [ ] Can mark complete/incomplete
- [ ] Can edit todos
- [ ] Can delete todos
- [ ] Filters work correctly
- [ ] Sort works correctly

---

### ⏰ Phase 8: Reminders & Scheduler

**Tujuan:** Sistem reminder dengan scheduler dan Windows notifications

**Tasks:**
1. Scheduler service:
   - Background thread untuk monitoring
   - Check reminders setiap menit
   - Handle missed reminders (catch-up logic)
   - Support untuk recurring reminders

2. Reminder operations:
   - Create reminder
   - Update reminder
   - Delete reminder
   - Snooze reminder (5, 10, 15 min options)
   - Mark complete

3. Windows Toast notifications:
   - Display toast when reminder triggers
   - Actions: Complete, Snooze, Open
   - Handle user responses

4. Recurrence rules:
   - Daily
   - Weekly (specific days)
   - Monthly

**Windows APIs:**
- `Windows.UI.Notifications.ToastNotification`
- `Windows.UI.Notifications.ToastNotificationManager`

**Testing Checklist:**
- [ ] Reminders trigger at correct time
- [ ] Toast notification appears
- [ ] Snooze works correctly
- [ ] Recurring reminders repeat
- [ ] Missed reminders show on startup
- [ ] Complete/dismiss works
- [ ] Handles system sleep/wake

---

### ⚙️ Phase 9: Settings & Configuration

**Tujuan:** UI untuk settings dan configuration management

**Tasks:**
1. Settings UI:
   - Position (left, center, right)
   - Monitor selection
   - Size settings
   - Animation preferences
   - Always-on-top toggle
   - Fullscreen behavior
   - Theme selection
   - Startup settings

2. Module toggles:
   - Enable/disable features
   - Music controls
   - Notifications
   - Reminders

3. Privacy settings:
   - Hide notification content
   - Hide music info
   - Quiet hours

4. Startup configuration:
   - Add to Windows startup
   - Registry integration

**Testing Checklist:**
- [ ] All settings save/load correctly
- [ ] Position changes work immediately
- [ ] Theme changes apply
- [ ] Startup setting works
- [ ] Module toggles work
- [ ] Settings persist after restart

---

### 🎤 Phase 10: Lyrics Integration (P1)

**Tujuan:** Display lyrics dengan sinkronisasi waktu

**Tasks:**
1. Lyrics provider integration:
   - Choose API (Genius, Musixmatch, etc.)
   - Implement search by title + artist
   - Parse LRC format for synced lyrics
   - Fallback to plain lyrics

2. Lyrics UI:
   - Display current line highlighted
   - Auto-scroll to current position
   - Manual scroll option
   - "Lyrics not found" state

3. Sync logic:
   - Match media position with lyric timestamps
   - Handle seeking in media
   - Smooth transitions

**Testing Checklist:**
- [ ] Lyrics fetch for known songs
- [ ] Synced lyrics highlight correctly
- [ ] Manual scroll works
- [ ] Handles lyrics not found
- [ ] Respects rate limits
- [ ] Works offline (cached lyrics)

---

### 📢 Phase 11: Notifications (P1)

**Tujuan:** Integrate Windows notifications dengan privacy controls

**Tasks:**
1. Notification listener:
   - Access notification history (if possible)
   - Listen for new notifications
   - Parse notification data

2. Notification display:
   - Show in dock
   - Grouped by app
   - Timestamp
   - Actions from original notification

3. Privacy controls:
   - Enable/disable per app
   - Hide content option
   - Show only indicator

**Windows APIs:**
- `Windows.UI.Notifications.Management.UserNotificationListener`
- Requires notification access permission

**Testing Checklist:**
- [ ] Detects new notifications
- [ ] Shows notification in dock
- [ ] Privacy controls work
- [ ] Can dismiss notifications
- [ ] App-specific settings work

---

### 🏁 Phase 12: Polish & Testing

**Tujuan:** Bug fixes, optimization, dan final testing

**Tasks:**
1. Performance optimization:
   - Profile CPU usage
   - Profile memory usage
   - Optimize rendering
   - Reduce background activity

2. Bug fixes:
   - Fix reported issues
   - Handle edge cases
   - Improve error messages

3. Multi-monitor support:
   - Test on multiple monitors
   - Handle monitor disconnect
   - Remember position per monitor

4. Documentation:
   - User guide
   - Keyboard shortcuts reference
   - Troubleshooting guide

5. Testing:
   - Manual testing all features
   - Edge case testing
   - Long-running stability test
   - Windows 10 vs 11 testing

**Testing Checklist:**
- [ ] All features work end-to-end
- [ ] No memory leaks
- [ ] CPU usage < 5% idle
- [ ] Handles errors gracefully
- [ ] Works on Windows 10 and 11
- [ ] Multi-monitor support solid
- [ ] Documentation complete

---

## Development Tips

### Debugging
```powershell
# Run with debug logs
$env:RUST_LOG="topdock=debug"; cargo run

# Run with trace logs (very verbose)
$env:RUST_LOG="topdock=trace"; cargo run
```

### Testing Windows APIs
Use `windows` crate examples and Microsoft docs:
- https://microsoft.github.io/windows-docs-rs/
- https://docs.microsoft.com/en-us/windows/win32/

### Performance Profiling
```powershell
# Build with profiling
cargo build --release --features profiling

# Use Windows Performance Analyzer
# Or Tracy profiler for detailed analysis
```

### Common Issues

**Window not appearing:**
- Check if window is created off-screen
- Verify monitor enumeration
- Check window styles

**High CPU usage:**
- Reduce redraw frequency
- Use event-driven updates, not polling
- Profile with `cargo flamegraph`

**Database locked:**
- Ensure connection pooling
- Check for unclosed transactions
- Use WAL mode for better concurrency

---

## Next Steps After MVP

1. **User Testing** - Get feedback from real users
2. **Installer** - Create Windows installer (MSI or NSIS)
3. **Auto-Update** - Implement update checking
4. **Plugins** - Plugin system for extensibility
5. **Themes** - Custom theme support
6. **Cloud Sync** - Optional cloud synchronization
7. **Mobile Companion** - Android/iOS apps

---

**Current Status:** Phase 1 Complete ✅
**Next:** Phase 2 - Floating Bar & Window Management

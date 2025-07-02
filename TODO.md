# AtlasConnect TODO

A comprehensive roadmap for building a high-performance, enterprise-grade remote access platform in Rust.

## 🎯 Vision
Build a premium MSP/enterprise remote access solution that combines:
- **ScreenConnect's enterprise workflow** - Professional web portal, client management, session types
- **RustDesk's performance** - Native high-performance capture, cross-platform optimization  
- **Modern architecture** - Leptos web UI, Rust backend, zero-trust security

---

## 📋 Phase 1: Core Client Architecture (Foundation)
*Priority: CRITICAL - Everything depends on this*

### Client Agent System
- [ ] **Service/Daemon Architecture**
  - [ ] Windows service integration (`windows-service` crate)
  - [ ] Linux systemd service 
  - [ ] macOS launchd daemon
  - [ ] Service installer/uninstaller
  - [ ] Auto-start on boot
  - [ ] Graceful shutdown handling

- [ ] **Agent Core**
  - [ ] `client/src/agent/mod.rs` - Main agent orchestrator
  - [ ] `client/src/agent/session_manager.rs` - Handle multiple concurrent sessions
  - [ ] `client/src/agent/heartbeat.rs` - Keep-alive with server
  - [ ] `client/src/agent/auto_update.rs` - Self-updating mechanism
  - [ ] `client/src/agent/installer.rs` - Deployment helpers

- [ ] **Connection Management**
  - [ ] `client/src/connection/mod.rs` - WebSocket client
  - [ ] `client/src/connection/relay.rs` - Server relay communication
  - [ ] `client/src/connection/auth.rs` - Agent authentication
  - [ ] `client/src/connection/reconnect.rs` - Auto-reconnection logic
  - [ ] Connection pooling for multiple sessions

---

## 📋 Phase 2: Screen Capture & Control (Core Performance)
*Priority: CRITICAL - The heart of remote access*

### Cross-Platform Screen Capture
- [ ] **Linux/Wayland + NVIDIA**
  - [ ] `client/src/capture/wayland.rs` - wlr-screencopy protocol
  - [ ] `client/src/capture/x11.rs` - X11 fallback (XRandR, XDamage)
  - [ ] NVIDIA NVENC hardware encoding integration
  - [ ] Multi-monitor support (Wayland outputs)
  - [ ] 60fps+ capture optimization

- [ ] **Windows Capture**
  - [ ] `client/src/capture/windows.rs` - DXGI 1.2+ screen capture
  - [ ] Windows.Graphics.Capture API (Win10+)
  - [ ] Hardware encoding (NVENC, Intel QSV, AMD VCE)
  - [ ] Multi-monitor enumeration
  - [ ] HDR display support

- [ ] **macOS Capture**
  - [ ] `client/src/capture/macos.rs` - Core Graphics/AVFoundation
  - [ ] VideoToolbox hardware encoding
  - [ ] Retina/HiDPI handling
  - [ ] Multiple display support
  - [ ] Screen recording permissions

### Input Control System
- [ ] **Cross-Platform Input**
  - [ ] `client/src/input/mod.rs` - Input abstraction layer
  - [ ] `client/src/input/linux.rs` - evdev, libinput integration
  - [ ] `client/src/input/windows.rs` - SendInput API
  - [ ] `client/src/input/macos.rs` - Core Graphics Events
  - [ ] Mouse movement, clicks, scroll
  - [ ] Keyboard input with proper key mapping
  - [ ] Multi-monitor coordinate mapping

### Video Encoding/Streaming
- [ ] **Hardware-Accelerated Encoding**
  - [ ] `client/src/encoding/mod.rs` - Encoder abstraction
  - [ ] `client/src/encoding/nvenc.rs` - NVIDIA NVENC
  - [ ] `client/src/encoding/qsv.rs` - Intel Quick Sync
  - [ ] `client/src/encoding/software.rs` - x264/x265 fallback
  - [ ] Adaptive bitrate based on network
  - [ ] Low-latency streaming optimizations

---

## 📋 Phase 3: Session Types & Enterprise Features
*Priority: HIGH - Core enterprise functionality*

### Session Management
- [ ] **Backstage Sessions (Unattended Admin)**
  - [ ] `client/src/session/backstage.rs` - Silent admin access
  - [ ] No user notifications/indicators
  - [ ] Full administrative privileges
  - [ ] Screen blanking capability
  - [ ] Auto-elevation (Windows UAC, Linux sudo, macOS admin)

- [ ] **Console Sessions (Interactive Remote)**
  - [ ] `client/src/session/console.rs` - Interactive control
  - [ ] User-visible remote session
  - [ ] Optional screen blanking
  - [ ] Input blocking controls
  - [ ] Session permission prompts

### Advanced Session Controls
- [ ] **Screen Blanking**
  - [ ] `client/src/session/screen_blank.rs` - Black out user screen
  - [ ] Windows: Custom desktop/secure desktop
  - [ ] Linux: Disable outputs, custom compositor layer
  - [ ] macOS: CGDisplayCapture, overlay window

- [ ] **Input Blocking** 
  - [ ] `client/src/session/input_block.rs` - Disable user input
  - [ ] Windows: Low-level hooks, disable input
  - [ ] Linux: Grab input devices, xinput disable
  - [ ] macOS: CGEventTap, input monitoring

- [ ] **Privilege Elevation**
  - [ ] `client/src/session/elevation.rs` - Run as admin/root
  - [ ] Windows: UAC elevation, token manipulation
  - [ ] Linux: sudo/pkexec integration
  - [ ] macOS: Authorization Services

---

## 📋 Phase 4: Web Portal Enhancement
*Priority: HIGH - Professional MSP interface*

### Enhanced Client Management
- [ ] **Client Dashboard Improvements**
  - [ ] Real-time client status indicators
  - [ ] Client grouping/tagging system
  - [ ] Custom client identifiers/names
  - [ ] Last seen timestamps
  - [ ] Performance metrics display

- [ ] **Session Launch Interface**
  - [ ] One-click Backstage/Console session launch
  - [ ] Session type selection modal
  - [ ] Advanced session options (screen blank, input block)
  - [ ] Multi-monitor selection
  - [ ] Session scheduling

### Client Information Panel
- [ ] **Detailed Device Info**
  - [ ] `server/src/web/components/client_info.rs` - Detailed client panel
  - [ ] System specifications (CPU, RAM, GPU, OS)
  - [ ] Network information (IP, location, ISP)
  - [ ] Installed software inventory
  - [ ] Hardware change detection
  - [ ] Performance graphs (CPU, RAM, network)

- [ ] **Session History**
  - [ ] Past session list with details
  - [ ] Session duration tracking
  - [ ] User activity logs
  - [ ] File transfer history
  - [ ] Audit trail for compliance

### Real-Time Communication
- [ ] **Chat/Messaging System**
  - [ ] `server/src/web/components/chat.rs` - In-session chat
  - [ ] Operator-to-client messaging
  - [ ] Message history persistence
  - [ ] File sharing via chat
  - [ ] Screen annotation tools

---

## 📋 Phase 5: Toolbox & Script Execution
*Priority: MEDIUM-HIGH - MSP automation features*

### Server-Side Script Management
- [ ] **Script Storage & Organization**
  - [ ] `server/src/models/script.rs` - Script database models
  - [ ] Script categories/folders
  - [ ] Version control for scripts
  - [ ] Script sharing between technicians
  - [ ] Permission-based script access

- [ ] **Script Editor Interface**
  - [ ] `server/src/web/components/script_editor.rs` - Web-based editor
  - [ ] Syntax highlighting for multiple languages
  - [ ] Script testing/validation
  - [ ] Parameter input forms
  - [ ] Execution history per script

### Remote Script Execution
- [ ] **Cross-Platform Execution**
  - [ ] `client/src/toolbox/mod.rs` - Script execution engine
  - [ ] `client/src/toolbox/powershell.rs` - Windows PowerShell
  - [ ] `client/src/toolbox/bash.rs` - Linux/macOS shell scripts
  - [ ] `client/src/toolbox/python.rs` - Python script support
  - [ ] Sandboxed execution environment
  - [ ] Real-time output streaming

- [ ] **Built-in Toolbox Scripts**
  - [ ] System information gathering
  - [ ] Network diagnostics (ping, traceroute, port scan)
  - [ ] Registry/config file editing
  - [ ] Software installation/removal
  - [ ] Windows updates, Linux package management
  - [ ] File system operations

---

## 📋 Phase 6: File Transfer & Management
*Priority: MEDIUM - Essential for support workflows*

### File Transfer System
- [ ] **Bidirectional File Transfer**
  - [ ] `client/src/transfer/mod.rs` - File transfer engine
  - [ ] Drag-and-drop upload interface
  - [ ] Multi-file/folder transfers
  - [ ] Transfer progress indicators
  - [ ] Resume interrupted transfers
  - [ ] Bandwidth throttling options

- [ ] **Remote File Manager**
  - [ ] `server/src/web/components/file_manager.rs` - Web file browser
  - [ ] Navigate remote file system
  - [ ] Create/delete/rename files and folders
  - [ ] File permissions editing
  - [ ] Quick file preview (text, images)
  - [ ] Compressed file handling

---

## 📋 Phase 7: Ad-Hoc Sessions & Quick Access
*Priority: MEDIUM - ScreenConnect-style quick access*

### One-Time Access Codes
- [ ] **Quick Access System**
  - [ ] `server/src/models/access_code.rs` - Temporary session codes
  - [ ] Generate time-limited access codes
  - [ ] Simple client download with embedded code
  - [ ] Auto-expiring temporary sessions
  - [ ] No permanent installation required

- [ ] **Client Launcher**
  - [ ] `client/src/launcher/mod.rs` - Temporary client mode
  - [ ] Portable executable (no installation)
  - [ ] Auto-connect with embedded credentials
  - [ ] Self-cleanup after session ends
  - [ ] Download from web portal

---

## 📋 Phase 8: Authentication & User Management
*Priority: MEDIUM - Enterprise security*

### Advanced Authentication
- [ ] **Multi-Factor Authentication**
  - [ ] TOTP (Google Authenticator) support
  - [ ] SMS verification option
  - [ ] Hardware key (FIDO2) support
  - [ ] Backup codes generation

- [ ] **SSO Integration**
  - [ ] SAML 2.0 support
  - [ ] OAuth2/OpenID Connect
  - [ ] Active Directory integration
  - [ ] LDAP authentication

### Role-Based Access Control
- [ ] **User Roles & Permissions**
  - [ ] `server/src/models/role.rs` - Role system
  - [ ] Technician/Administrator/Viewer roles
  - [ ] Client group permissions
  - [ ] Session type restrictions
  - [ ] Feature-based access control

---

## 📋 Phase 9: Performance & Optimization
*Priority: MEDIUM - RustDesk-level performance*

### Network Optimization
- [ ] **Advanced Streaming**
  - [ ] Adaptive quality based on bandwidth
  - [ ] Delta compression for screen updates
  - [ ] Region-of-interest encoding
  - [ ] Predictive frame caching
  - [ ] Network jitter compensation

- [ ] **Protocol Optimization**
  - [ ] Custom binary protocol over WebSocket
  - [ ] Message compression (zstd, lz4)
  - [ ] Connection multiplexing
  - [ ] UDP hole punching for P2P
  - [ ] TURN server fallback

### Resource Management
- [ ] **Memory & CPU Optimization**
  - [ ] Zero-copy screen capture where possible
  - [ ] Efficient frame buffering
  - [ ] Multi-threaded encoding pipeline
  - [ ] Resource usage monitoring
  - [ ] Automatic quality degradation under load

---

## 📋 Phase 10: Deployment & DevOps
*Priority: LOW-MEDIUM - Production readiness*

### Installation & Deployment
- [ ] **Client Installers**
  - [ ] Windows MSI installer
  - [ ] Linux packages (deb, rpm, snap)
  - [ ] macOS pkg installer
  - [ ] Silent/unattended installation
  - [ ] Group Policy deployment (Windows)

- [ ] **Server Deployment**
  - [ ] Docker Compose for development
  - [ ] Kubernetes manifests
  - [ ] Helm charts
  - [ ] Terraform modules for cloud deployment
  - [ ] Auto-scaling configuration

### Monitoring & Logging
- [ ] **Observability**
  - [ ] Structured logging (tracing/log)
  - [ ] Metrics collection (Prometheus)
  - [ ] Health checks and monitoring
  - [ ] Performance profiling
  - [ ] Error tracking and alerting

- [ ] **Audit & Compliance**
  - [ ] Session recording/playback
  - [ ] Detailed audit logs
  - [ ] Compliance reporting
  - [ ] Data retention policies
  - [ ] GDPR/CCPA compliance tools

---

## 📋 Phase 11: Advanced Features
*Priority: LOW - Nice-to-have enterprise features*

### Enhanced Session Features
- [ ] **Multi-Session Management**
  - [ ] Concurrent session handling
  - [ ] Session handoff between technicians
  - [ ] Collaborative sessions (multiple operators)
  - [ ] Session recording and playback
  - [ ] Screen annotation tools

- [ ] **Advanced Remote Tools**
  - [ ] Remote registry editor (Windows)
  - [ ] Process manager (kill, start processes)
  - [ ] Service management
  - [ ] Event log viewer
  - [ ] Performance monitoring tools

### Integration & APIs
- [ ] **External Integrations**
  - [ ] REST API for third-party tools
  - [ ] Webhook notifications
  - [ ] Ticketing system integration
  - [ ] CRM/PSA integrations
  - [ ] Mobile app for technicians

---

## 🎯 Development Priorities

### Phase 1-2 (Foundation): Weeks 1-4
- Client service architecture
- Basic screen capture and input control
- WebSocket communication

### Phase 3-4 (Core Features): Weeks 5-8  
- Backstage/Console session types
- Screen blanking and input blocking
- Enhanced web portal

### Phase 5-6 (MSP Features): Weeks 9-12
- Toolbox and script execution
- File transfer system
- Client management improvements

### Phase 7-8 (Access & Security): Weeks 13-16
- Ad-hoc sessions
- Advanced authentication
- Role-based access control

### Phase 9-11 (Polish & Production): Weeks 17-20
- Performance optimization
- Deployment tools
- Advanced enterprise features

---

## 🔧 Technical Debt & Refactoring
- [ ] Error handling standardization
- [ ] Comprehensive test suite
- [ ] Documentation completion
- [ ] Code review and cleanup
- [ ] Security audit and penetration testing
- [ ] Performance benchmarking and optimization

---

*Last updated: July 2, 2025*

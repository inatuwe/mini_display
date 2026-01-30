# Tasks: Display FS V1 (0.96 inch) - Implementation Tasks

## Overview
Small, autonomous tasks to implement the Display FS V1 interaction program.  
**Approach:** Test-Driven Development (TDD) - Write tests first, then implement.

---

## Task 1: Project Setup ✅
**Goal:** Create project structure and dependencies

### 1.1 Create requirements.txt
- [x] Test: File exists and contains required packages
- [x] Implement: Create `requirements.txt` with `pyserial>=3.5` and `pillow>=9.0.0`

### 1.2 Create project structure
- [x] Test: Directories exist
- [x] Implement: Create `src/` and `tests/` directories

### 1.3 Install dependencies
- [x] Test: `import serial` and `import PIL` work without error
- [x] Implement: Run `pip install -r requirements.txt`

---

## Task 2: COM Port Enumeration ✅
**Goal:** List all available COM ports on the system

### 2.1 Write test for port listing function
- [x] Test: Function returns a list (can be empty)
- [x] Test: Each item in list has `device`, `description`, `hwid` attributes

### 2.2 Implement port listing function
- [x] Implement: `list_com_ports()` that returns all available COM ports
- [x] Verify: Test passes

### 2.3 Write test for port details formatting
- [x] Test: Function formats port info as readable string
- [x] Test: Output contains port name and description

### 2.4 Implement port details formatting
- [x] Implement: `format_port_info(port)` function
- [x] Verify: Test passes

---

## Task 3: Display Detection ✅
**Goal:** Detect if Display FS V1 is connected

### 3.1 Write test for display detection by VID/PID
- [x] Test: Function returns True if known VID/PID found
- [x] Test: Function returns False if no matching device
- [x] Test: Function handles empty port list

### 3.2 Implement display detection by VID/PID
- [x] Implement: `is_display_fs_connected()` function
- [x] Verify: Test passes

### 3.3 Write test for finding display port
- [x] Test: Function returns port object if found
- [x] Test: Function returns None if not found

### 3.4 Implement find display port function
- [x] Implement: `find_display_port()` function
- [x] Verify: Test passes

---

## Task 4: Serial Connection ✅
**Goal:** Establish serial connection to display

### 4.1 Write test for connection opening
- [x] Test: Function opens connection with correct baud rate (115200)
- [x] Test: Function raises exception on invalid port
- [x] Test: Function handles timeout parameter

### 4.2 Implement connection opening
- [x] Implement: `open_connection(port, baud_rate=115200)` function
- [x] Verify: Test passes

### 4.3 Write test for connection closing
- [x] Test: Function closes open connection
- [x] Test: Function handles already closed connection gracefully

### 4.4 Implement connection closing
- [x] Implement: `close_connection(connection)` function
- [x] Verify: Test passes

---

## Task 5: Image Creation ✅
**Goal:** Create image with text for display

### 5.1 Write test for blank image creation
- [x] Test: Function creates image with specified dimensions
- [x] Test: Function creates image with specified background color
- [x] Test: Default size matches display (160x80 or similar)

### 5.2 Implement blank image creation
- [x] Implement: `create_blank_image(width, height, bg_color)` function
- [x] Verify: Test passes

### 5.3 Write test for text drawing
- [x] Test: Function adds text to image
- [x] Test: Function accepts text position parameters
- [x] Test: Function accepts font size parameter

### 5.4 Implement text drawing
- [x] Implement: `draw_text(image, text, position, font_size)` function
- [x] Verify: Test passes

### 5.5 Write test for "Hello World!" image
- [x] Test: Function creates complete image with "Hello World!" text
- [x] Test: Image has correct dimensions

### 5.6 Implement "Hello World!" image creation
- [x] Implement: `create_hello_world_image()` function
- [x] Verify: Test passes

---

## Task 6: Image to Bytes Conversion
**Goal:** Convert PIL image to bytes for serial transmission

### 6.1 Write test for image to RGB bytes conversion
- [ ] Test: Function converts image to byte array
- [ ] Test: Output size matches width × height × bytes_per_pixel

### 6.2 Implement image to bytes conversion
- [ ] Implement: `image_to_bytes(image)` function
- [ ] Verify: Test passes

---

## Task 7: Send Data to Display
**Goal:** Send image data to display via serial

### 7.1 Write test for sending bytes
- [ ] Test: Function sends bytes to serial connection
- [ ] Test: Function handles connection errors
- [ ] Test: Function returns success/failure status

### 7.2 Implement send bytes function
- [ ] Implement: `send_bytes(connection, data)` function
- [ ] Verify: Test passes

### 7.3 Write test for display command protocol (if needed)
- [ ] Test: Function wraps data with correct header/footer
- [ ] Test: Function includes correct command bytes

### 7.4 Implement display command protocol
- [ ] Implement: `create_display_command(image_data)` function
- [ ] Verify: Test passes

---

## Task 8: Main Detection Script
**Goal:** Create standalone detection script

### 8.1 Write integration test for detection script
- [ ] Test: Script runs without error
- [ ] Test: Script outputs connection status message

### 8.2 Implement detect_display.py
- [ ] Implement: Main script that detects and reports display status
- [ ] Verify: Test passes

---

## Task 9: Main Hello World Script
**Goal:** Create standalone hello world script

### 9.1 Write integration test for hello world script
- [ ] Test: Script runs without error when display connected
- [ ] Test: Script shows error message when display not connected

### 9.2 Implement hello_world.py
- [ ] Implement: Main script that displays "Hello World!"
- [ ] Verify: Test passes

---

## Task 10: Error Handling & Polish
**Goal:** Add robust error handling

### 10.1 Write tests for error cases
- [ ] Test: Graceful handling of disconnected display
- [ ] Test: Graceful handling of permission errors
- [ ] Test: Helpful error messages for users

### 10.2 Implement error handling
- [ ] Implement: Try/except blocks with user-friendly messages
- [ ] Verify: All tests pass

---

## Execution Order

```
Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → Task 6 → Task 7 → Task 8 → Task 9 → Task 10
  ↓        ↓        ↓        ↓        ↓        ↓        ↓        ↓        ↓        ↓
Setup   Ports   Detect  Connect  Image   Convert  Send   Script1  Script2  Polish
```

---

## File Structure After Completion

```
mini_display/
├── PLAN_display_fs_v1.md
├── TASKS_display_fs_v1.md
├── requirements.txt
├── src/
│   ├── __init__.py
│   ├── port_utils.py        # Tasks 2, 3
│   ├── connection.py        # Task 4
│   ├── image_utils.py       # Tasks 5, 6
│   ├── display_protocol.py  # Task 7
│   ├── detect_display.py    # Task 8
│   └── hello_world.py       # Task 9
└── tests/
    ├── __init__.py
    ├── test_port_utils.py
    ├── test_connection.py
    ├── test_image_utils.py
    ├── test_display_protocol.py
    └── test_integration.py
```

---

## Notes

- Each task has test written BEFORE implementation
- Tasks are small enough to complete in one session
- Tasks build on each other sequentially
- Mock serial connections for unit tests (don't require hardware)
- Integration tests (Tasks 8, 9) require actual display

**Start with Task 1 when ready!**

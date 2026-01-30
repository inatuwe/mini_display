# Plan 002: Fix Python Test Failures

**Status:** COMPLETED

## Summary

Fix 3 failing tests in the Python implementation. The main scripts work correctly, but some tests have incorrect expectations.

## Issues Found

### Issue 1: `test_hello_world.py` - Wrong mock target (2 tests)

**Files:** `tests/test_hello_world.py`

**Problem:** Tests try to patch `hello_world.send_bytes` but `hello_world.py` imports and uses `send_image_to_display` instead, not `send_bytes`.

**Failing tests:**

- `test_script_runs_without_error_when_connected`
- `test_script_creates_and_sends_image`

**Fix:** Update tests to patch `hello_world.send_image_to_display` instead of `hello_world.send_bytes`.

### Issue 2: `test_image.py` - Wrong byte order expectation (1 test)

**File:** `tests/test_image.py`

**Problem:** `test_red_pixel_converts_correctly` expects big-endian (`b'\xf8\x00'`) but `image_to_bytes()` correctly produces little-endian (`b'\x00\xf8'`) as documented.

**Failing test:**

- `test_red_pixel_converts_correctly`

**Fix:** Update test to expect little-endian byte order: `b'\x00\xf8'`

## Tasks

- [x] Fix `tests/test_hello_world.py` - Change mock from `send_bytes` to `send_image_to_display`
- [x] Fix `tests/test_image.py` - Update expected byte order to little-endian

## Verification

```bash
python3 -m pytest tests/ -v
```

All 61 tests should pass.

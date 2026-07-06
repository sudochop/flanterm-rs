#!/usr/bin/bash

bindgen \
    --use-core \
    --allowlist-file flanterm/src/flanterm.h \
    --allowlist-file flanterm/src/flanterm_backends/fb.h \
    bindgen-wrapper.h \
    -- -Iflanterm/src \
    > src/sys.rs

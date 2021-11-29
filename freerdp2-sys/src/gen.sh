#!/bin/sh

bindgen -o bindgen.rs freerdp.h  \
	--allowlist-function '.*Channel.*' \
	--allowlist-function 'Create.*' \
	--allowlist-function 'Get.*' \
	--allowlist-function 'PubSub.*' \
	--allowlist-function 'WLog.*' \
	--allowlist-function 'client_.*' \
	--allowlist-function 'freerdp_.*' \
	--allowlist-function 'FreeRDP.*' \
	--allowlist-function 'cliprdr_.*' \
	--allowlist-function 'gdi_.*' \
	--allowlist-function 'rdpgfx_.*' \
	--allowlist-function 'graphics_.*' \
	--allowlist-function 'stream_.*' \
	--allowlist-function 'Wait.*' \
	--allowlist-type 'Rdp.*' \
	--allowlist-type 'Disp.*' \
	--allowlist-var 'AUDIN_.*' \
	--allowlist-var 'CB_.*' \
	--allowlist-var 'CLIPRDR_.*' \
	--allowlist-var 'CONNECTION_.*' \
	--allowlist-var 'FreeRDP.*' \
	--allowlist-var 'RDP.*' \
	--allowlist-var 'PIXEL_.*' \
	--allowlist-var 'OS.*' \
	--allowlist-var 'WAIT_.*' \
	-- `pkg-config --cflags freerdp2`

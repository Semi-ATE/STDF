import struct
import tempfile

import pytest

from Semi_ATE import STDF
from Semi_ATE.STDF import STDR
from Semi_ATE.STDF.STDR import _safe_decode


def test_STDR():
    
    stdr = STDR()

#   test unsigned byte value    
    ubyte_field = 5
    set_ubyte = 5;
    stdr.set_value(ubyte_field, set_ubyte)
    get_ubyte = stdr.get_value(5)
    assert set_ubyte == get_ubyte

    try:
        stdr.set_value(ubyte_field, -5)
        assert False
    except:
        assert True

    try:
        stdr.set_value(ubyte_field, 500)
        assert False
    except:
        assert True


# --- _safe_decode helper (regression for issue #77) ---


def test_safe_decode_plain_utf8():
    assert _safe_decode(b"hello world") == "hello world"


def test_safe_decode_falls_back_to_latin1_for_cp1252_en_dash():
    """0x96 is the en-dash in CP-1252 / Windows-1252 and an invalid UTF-8
    start byte. Teradyne Ultraflex emits these in test descriptions
    (issue #77). Latin-1 fallback maps the byte to U+0096 so the field
    stays legible and parsing of the STDF file continues."""
    raw = b"Test Suite \x96 CONTINUITY"
    result = _safe_decode(raw)
    assert isinstance(result, str)
    assert "Test Suite" in result
    assert "CONTINUITY" in result
    assert len(result) == len(raw)  # one byte → one code point under latin-1


def test_safe_decode_accepts_every_byte_value():
    """latin-1 spans the full 0x00..0xFF range, so _safe_decode must never
    raise UnicodeDecodeError regardless of input bytes."""
    raw = bytes(range(256))
    result = _safe_decode(raw)
    assert len(result) == 256


def test_safe_decode_ascii_path_falls_back():
    """The ASCII code path (GDR record codes 10/11) must also fall back."""
    raw = b"hello\xc3"  # 0xc3 is not valid ASCII
    result = _safe_decode(raw, encoding="ASCII")
    assert isinstance(result, str)
    assert result.startswith("hello")


@pytest.mark.parametrize("encoding", ["utf-8", "ASCII"])
def test_safe_decode_pure_ascii_round_trip(encoding):
    """Pure-ASCII payload decodes identically under any encoding setting."""
    assert _safe_decode(b"plain ascii lot_id", encoding) == "plain ascii lot_id"


def test_records_from_file_handles_legacy_ansi_byte_in_c_n_field():
    """End-to-end regression for #77: an STDF file with a 0x96 byte inside
    a C*n field (WAFER_ID here, analogous to TEST_TXT in the reporter's
    Teradyne Ultraflex case) must parse cleanly through the full
    records_from_file pipeline, not raise UnicodeDecodeError mid-stream."""
    # FAR header (V4 little-endian) — written by STDF.FAR().__repr__()
    far_bytes = STDF.FAR().__repr__()

    # WIR with WAFER_ID = "LOT\x96123" — 0x96 in the middle of the lot id
    wafer_id = b"LOT\x96123"
    head_num = 1
    site_grp = 1
    start_t = 1609462861
    wir_body = struct.pack("<BBI", head_num, site_grp, start_t) + \
        bytes([len(wafer_id)]) + wafer_id
    wir_header = struct.pack("<HBB", len(wir_body), 2, 10)  # REC_LEN, REC_TYP=2, REC_SUB=10
    wir_bytes = wir_header + wir_body

    with tempfile.NamedTemporaryFile(mode="wb", suffix=".stdf", delete=False) as f:
        f.write(far_bytes + wir_bytes)
        file_path = f.name

    records = list(STDF.records_from_file(file_path))
    assert len(records) == 2  # FAR + WIR
    wir = records[1]
    wafer_id_decoded = wir.get_value("WAFER_ID")
    assert isinstance(wafer_id_decoded, str)
    assert "LOT" in wafer_id_decoded
    assert "123" in wafer_id_decoded

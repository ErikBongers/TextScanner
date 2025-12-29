
import text_scanner_py

res = text_scanner_py.scan_wpl(r"Z:\Music\My Playlists\Religioso.wpl")
for line in res:
    print(line)

def test_it():
    print("test")
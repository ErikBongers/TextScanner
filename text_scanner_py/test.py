
import text_scanner_py

res = text_scanner_py.scan_wpl(r"Z:\Music\My Playlists\Gamba.wpl")
print(res)
print(res.name)
for line in res.items:
    print(line.path)

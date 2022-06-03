curl -s https://api.github.com/repos/NalinPlad/plus/releases/latest \
| grep "browser_download_url.*deb" \
| cut -d : -f 2,3 \
| tr -d \" \
| curl -o -
set -e
rm -rf build/
mkdir -p build/

# GOOS=linux GOARCH=arm GOARM=7 go build -o build/blog-backend ## build raspberry
GOOS=linux GOARCH=amd64 go build -o build/blog-backend

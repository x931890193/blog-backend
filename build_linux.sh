set -e
rm -rf build/
mkdir -p build/

GOOS=linux GOARCH=amd64 go build -o build/blog-backend

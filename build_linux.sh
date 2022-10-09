set -e
rm -rf build/
mkdir -p build/
cp -r config/.config.yml build/
GOOS=linux GOARCH=amd64 go build -o build/blog-backend

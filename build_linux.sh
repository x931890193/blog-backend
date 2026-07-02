set -e
rm -rf build/
mkdir -p build/

GOOS=${GOOS:-linux}
GOARCH=${GOARCH:-arm}
GOARM=${GOARM:-7}

GOOS=$GOOS GOARCH=$GOARCH GOARM=$GOARM go build -o build/blog-backend

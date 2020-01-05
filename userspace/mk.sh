. ../mk/colors.sh

rm -rf build
mkdir -p build/libs
mkdir -p build/bins

# Compile libs
for libSrc in $(ls "src/libs") ; do
  echo "    ${ORANGE}Compile library userspace/src/libs/$libSrc${NORMAL}" | tr -d "'"
  make -s -C "src/libs/$libSrc"
done

# Compile libs
for binSrc in $(ls "src/bins") ; do
  echo "    ${ORANGE}Compile binary userspace/src/bins/$binSrc${NORMAL}" | tr -d "'"
  make -s -C "src/bins/$binSrc"
done

# Copy binaries
for build in $(ls "build/bins") ; do
  echo "    ${ORANGE}Copy userspace/build/bins/$build to root/bin/$build${NORMAL}" | tr -d "'"
  cp "build/bins/$build" "../root/bin/$build"
done

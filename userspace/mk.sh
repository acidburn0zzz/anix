. ../mk/colors.sh

# TODO: Create a config file for each software to know if it is a library or a
# binary
rm -rf build
mkdir build

# Compile
for src in $(ls "src") ; do
  echo "    ${ORANGE}Compile userspace/src/$src${NORMAL}" | tr -d "'"
  make -C "src/$src" 1> /dev/null 2> /dev/null
done

# Copy
for build in $(ls "build") ; do
  echo "    ${ORANGE}Copy userspace/build/$build to root/bin/$build${NORMAL}" | tr -d "'"
  cp "build/$build" "../root/bin/$build"
done

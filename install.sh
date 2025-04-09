#!/usr/bin/env sh

base_dir="${1}"

if [ -z "${base_dir}" ]
then
    echo "missing base directory"
    echo "Usage:"
    echo "    ./install.sh <base_directory>"

	exit
fi

if ! [ -e "target/release/bedrock" ]
then
	echo "executable has not been build"
	echo "note: build using \`cargo +nightly build --release\`"

	exit
fi

notify() {
	echo "${@}"
	"${@}"
}

notify mkdir -pm755 "${base_dir}/usr/bin"
notify mkdir -pm755 "${base_dir}/usr/share/applications"
notify mkdir -pm755 "${base_dir}/usr/share/pixmaps"

notify install -m755 "target/release/bedrock" "${base_dir}/usr/bin/bedrock"
notify install -m644 "bedrock.svg" "${base_dir}/usr/share/pixmaps/bedrock.svg"

notify desktop-file-install --dir="${base_dir}/usr/share/applications" "bedrock.desktop"

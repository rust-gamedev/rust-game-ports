#!/bin/bash

set -o pipefail
set -o errexit
set -o nounset
set -o errtrace
shopt -s inherit_errexit

c_reference_project=boing-ggez
v_project_name=

function decode_cmdline_args {
  if [[ $# -ne 1 || $1 == "-h" || $1 == "--help" ]]; then
    echo "Usage: $(basename "$0") <project_name>"
    exit 0
  fi

  v_project_name=$1
}

function move_to_root_dir {
  cd "$(dirname "$0")/.."
}

function check_project_name {
  if [[ ! -d original_code/$v_project_name-master ]]; then
    >&2 echo 'Project original code not found!'
    exit 1
  fi
}

function add_project {
  perl -i -0777 -pe "s/]$/  \"$v_project_name\",\n]/" Cargo.toml
  cargo new "$v_project_name"
  rsync -av --relative "$c_reference_project/./Cargo.toml" "$c_reference_project/./rust-toolchain" "$c_reference_project/./.cargo" "$v_project_name/"
  perl -i -pe "s/$c_reference_project/$v_project_name/" "$v_project_name/Cargo.toml"
  git ca -m "Add ${v_project_name^} project"
}

function move_original_code {
  mv "original_code/$v_project_name-master" "$v_project_name/resources"
  cp "$v_project_name/resources/$v_project_name.py" "$v_project_name/src"
  sed -i '1i #!/usr/bin/env python3' "$v_project_name/resources/$v_project_name.py"
  chmod 755 "$v_project_name/resources/$v_project_name.py"
  git ca -m "${v_project_name^}: Move original project inside the Rust project"
}

decode_cmdline_args "$@"
move_to_root_dir
check_project_name
add_project
move_original_code

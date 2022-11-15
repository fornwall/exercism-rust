#!/bin/sh
set -e -u

if [ $# = 1 ]; then
    DIRECTORIES=$1
else
    DIRECTORIES=*/
fi

CLIPPY_PARAMS="--all-targets -- \
 -W clippy::cast_lossless \
 -W clippy::dbg_macro \
 -W clippy::expect_used \
 -W clippy::manual_filter_map \
 -W clippy::if_not_else \
 -W clippy::items_after_statements \
 -W clippy::large_stack_arrays \
 -W clippy::linkedlist \
 -W clippy::match_same_arms \
 -W clippy::nursery \
 -W clippy::option_if_let_else \
 -W clippy::redundant_closure_for_method_calls \
 -W clippy::needless_continue \
 -W clippy::needless_pass_by_value \
 -W clippy::semicolon_if_nothing_returned \
 -W clippy::similar_names \
 -W clippy::single_match_else \
 -W clippy::trivially_copy_pass_by_ref \
 -W clippy::unreadable-literal \
 -W clippy::unseparated-literal-suffix \
 -W clippy::unwrap_used \
 -D warnings"

for directory in $DIRECTORIES; do
  echo "$directory"
  cd "$directory"

  cargo fmt

  cargo clippy $CLIPPY_PARAMS

  cargo test

  if grep -F '[ignore]' tests/*.rs; then
       echo Found ignored tests
       exit 1
  fi

  cd ..
done



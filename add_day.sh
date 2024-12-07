#! /bin/bash -e

DAY=$(printf "%02d" $1)
export CRATE=day${DAY}

# add crate to workspace members
sed -i -e '/# placeholder/i "'${CRATE}'",' Cargo.toml

# create new crate
cargo new $CRATE
cargo add --package ${CRATE} --path common

# update the base empty template
cat empty_day.rs.template | envsubst > ${CRATE}/src/main.rs

# check that things build
cargo build

cd $CRATE ; aoc -d ${DAY} d ; cd ..


git add Cargo.toml
git add $CRATE
git commit -m "feat: adding ${CRATE}"


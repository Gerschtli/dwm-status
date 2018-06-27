#!/usr/bin/env bash
set -e

if [[ "${DRY_RUN}" != "false" ]]; then
    exit 0
fi

CHANGELOG="CHANGELOG.md"
CURRENT_DATE="$(date "+%Y-%m-%d")"

sed -E -i \
    -e "s,^(## \[Unreleased\])$,\1\n\n## [${NEW_VERSION}] - ${CURRENT_DATE}," \
    -e "s,^(\[Unreleased\]:)(.*?)${PREV_VERSION}(\.\.\.HEAD)$,\1\2${NEW_VERSION}\3\n[${NEW_VERSION}]:\2${PREV_VERSION}...${NEW_VERSION}," \
    "${CHANGELOG}"

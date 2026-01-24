export OLSON_DIR=./tzdata
export PRODUCT_ID=-//github.com/lennart-k/vzic-rs//RustiCal Calendar server//EN
export TZID_PREFIX=
export CREATE_SYMLINK=0
export IGNORE_TOP_LEVEL_LINK=0

tzdata_ics: tzdata libical/vzic/vzic
	./libical/vzic/vzic --dump --pure --output-dir tzdata_ics --olson-dir tzdata

libical/vzic/vzic:
	make -C libical/vzic vzic


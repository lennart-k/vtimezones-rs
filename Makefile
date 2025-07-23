export OLSON_DIR=./tzdata
export PRODUCT_ID=-//github.com/lennart-k/vzic-rs//RustiCal Calendar server//EN
export TZID_PREFIX=
export CREATE_SYMLINK=1

tzdata_ics: tzdata vzic/vzic
	./vzic/vzic --dump --pure --output-dir tzdata_ics --olson-dir tzdata

vzic/vzic:
	make -C vzic vzic


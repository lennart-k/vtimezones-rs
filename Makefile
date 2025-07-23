OLSON_DIR="./tzdata"
PRODUCT_ID="-//github.com/lennart-k/vzic-rs//RustiCal Calendar server//EN"
TZID_PREFIX=""
CREATE_SYMLINK="1"

tzdata_ics: tzdata vzic/vzic
	./vzic/vzic --dump --pure --output-dir tzdata_ics --olson-dir tzdata

vzic/vzic:
	make -C vzic vzic


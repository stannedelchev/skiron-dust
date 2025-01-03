NOW=`date +%F-%H-%M`
LOAD=$NOW'_dust_load.gif'
CONCENTRATION=$NOW'_dust_concentration.gif'

./skiron-dust --dust-load $LOAD --dust-concentration $CONCENTRATION --fps 3

cp $LOAD latest_load.gif
cp $CONCENTRATION latest_concentration.gif
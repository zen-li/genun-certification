for ((i=2000; i<=2200; i++))
do 
    IDENTITY = $(dfx identity remove "user$i")
    echo $IDENTITY
done
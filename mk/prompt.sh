while true; do
    read -p "Are you sure to continue ?" yn
    case $yn in
        [Yy]* ) break;;
        [Nn]* ) killall make;;
        * ) echo "Please answer y for yes or n for no.";;
    esac
done

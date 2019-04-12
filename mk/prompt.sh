while true; do
    read -p "Do you want to continue ?" yn
    case $yn in
        [Yy]* ) break;;
        [Nn]* ) killall make;;
        * ) echo "Please answer y for yes or n for no.";;
    esac
done

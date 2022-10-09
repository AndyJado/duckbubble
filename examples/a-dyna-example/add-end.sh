for i in */*.k; do
  echo $i;
  (echo "*END" && cat $i) >temp && mv temp $i;
done
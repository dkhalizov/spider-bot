kubectl delete deployment spider-bot -n spider-bot
kubectl delete job db-init -n spider-bot
kubectl delete pvc sqlite-data -n spider-bot
helm upgrade --install spider-bot . --namespace spider-bot -f values.yaml

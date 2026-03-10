## Run
```
ZENCH=warn cargo test --release --package cpu_stability_trait_for_report -- --no-capture
```

## Results

Screenshots from the [Fedora](https://www.fedoraproject.org/workstation/) System Monitor, showing CPU usage and the results of the Zench tests.

The command used to generate CPU stress is:

```
stress-ng --cpu 6 --cpu-load 100 -t 3600
stress-ng --cpu 12 --cpu-load 10 -t 3600
stress-ng --cpu 11 --cpu-load 100 -t 3600

stress-ng --cpu 12 --cpu-load 50 -t 3600
```

### System stable > zench test passes

```
stress-ng --cpu 6 --cpu-load 100 -t 3600
```
![alt text](<img/z_6_100p.min.png>)


```
stress-ng --cpu 12 --cpu-load 10 -t 3600
```
![alt text](<img/z_12_10p.min.png>)


```
stress-ng --cpu 11 --cpu-load 100 -t 3600
```
![alt text](<img/z_11_100p.min.png>)

### System unstable > zench test fails

```
stress-ng --cpu 12 --cpu-load 50 -t 3600
```
![alt text](<img/z_12_50p.min.png>)

# wpt2gpx

A simple command line tool to convert a wpt file to a gpx file. It reads wpt from stdin and writes gpx to stdout.


wpt
```
$FormatGEO
AAT054    N 36 16 36.42    E 140 08 43.50   540  ASIO HG TO
ACT052    N 36 16 10.11    E 140 08 28.70   524  COO TO
...
```

gpx
```
<gpx xmlns="http://www.topografix.com/GPX/1/0" version="1.0">
   <wpt lat="36.276783" lon="140.145417">
      <ele>540.0</ele>
      <name>AAT054 ASIO HG TO</name>
   </wpt>
   <wpt lat="36.269475" lon="140.141306">
      <ele>524.0</ele>
      <name>ACT052 COO TO</name>
   </wpt>
   ...
</gpx>
```

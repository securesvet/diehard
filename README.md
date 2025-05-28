# Diehard tests

Diehard tests written in Rust

## Prerequisites

Install rust and `dieharder cli`

UBUNTU: `sudo apt-get install dieharder`

## HOW TO RUN

Run with `cargo build && cargo run`, you will get `output.bin` file, which you can use in order to pass it to the `dieharder` cli with the command:

```
cat output.bin | dieharder -g 200 -a
```

## Ran tests with `dieharder`

```
#=============================================================================#
#            dieharder version 3.31.1 Copyright 2003 Robert G. Brown          #
#=============================================================================#
   rng_name    |rands/second|   Seed   |
stdin_input_raw|  5.68e+07  |2788688389|
#=============================================================================#
        test_name   |ntup| tsamples |psamples|  p-value |Assessment
#=============================================================================#
   diehard_birthdays|   0|       100|     100|0.44298183|  PASSED  
      diehard_operm5|   0|   1000000|     100|0.04160797|  PASSED  
  diehard_rank_32x32|   0|     40000|     100|0.93516261|  PASSED  
    diehard_rank_6x8|   0|    100000|     100|0.64500293|  PASSED  
   diehard_bitstream|   0|   2097152|     100|0.40858095|  PASSED  
        diehard_opso|   0|   2097152|     100|0.10192198|  PASSED  
        diehard_oqso|   0|   2097152|     100|0.48018235|  PASSED  
         diehard_dna|   0|   2097152|     100|0.08640311|  PASSED  
diehard_count_1s_str|   0|    256000|     100|0.89286378|  PASSED  
diehard_count_1s_byt|   0|    256000|     100|0.45100338|  PASSED  
 diehard_parking_lot|   0|     12000|     100|0.38297064|  PASSED  
    diehard_2dsphere|   2|      8000|     100|0.29589077|  PASSED  
    diehard_3dsphere|   3|      4000|     100|0.68697495|  PASSED  
     diehard_squeeze|   0|    100000|     100|0.28288037|  PASSED  
        diehard_sums|   0|       100|     100|0.67195221|  PASSED  
        diehard_runs|   0|    100000|     100|0.70921550|  PASSED  
        diehard_runs|   0|    100000|     100|0.28513776|  PASSED  
       diehard_craps|   0|    200000|     100|0.98069917|  PASSED  
       diehard_craps|   0|    200000|     100|0.97206950|  PASSED  
 marsaglia_tsang_gcd|   0|  10000000|     100|0.78613349|  PASSED  
 marsaglia_tsang_gcd|   0|  10000000|     100|0.92923695|  PASSED  
         sts_monobit|   1|    100000|     100|0.21324541|  PASSED  
            sts_runs|   2|    100000|     100|0.72091929|  PASSED  
          sts_serial|   1|    100000|     100|0.67794259|  PASSED  
          sts_serial|   2|    100000|     100|0.98014716|  PASSED  
          sts_serial|   3|    100000|     100|0.57767096|  PASSED  
          sts_serial|   3|    100000|     100|0.69615745|  PASSED  
          sts_serial|   4|    100000|     100|0.57617293|  PASSED  
          sts_serial|   4|    100000|     100|0.55028322|  PASSED  
          sts_serial|   5|    100000|     100|0.71132992|  PASSED  
          sts_serial|   5|    100000|     100|0.97648066|  PASSED  
          sts_serial|   6|    100000|     100|0.73273194|  PASSED  
          sts_serial|   6|    100000|     100|0.62708785|  PASSED  
          sts_serial|   7|    100000|     100|0.61858295|  PASSED  
          sts_serial|   7|    100000|     100|0.51172745|  PASSED  
          sts_serial|   8|    100000|     100|0.21914370|  PASSED  
          sts_serial|   8|    100000|     100|0.82091098|  PASSED  
          sts_serial|   9|    100000|     100|0.80362581|  PASSED  
          sts_serial|   9|    100000|     100|0.37933640|  PASSED  
          sts_serial|  10|    100000|     100|0.82480841|  PASSED  
          sts_serial|  10|    100000|     100|0.64185402|  PASSED  
          sts_serial|  11|    100000|     100|0.67121068|  PASSED  
          sts_serial|  11|    100000|     100|0.67075764|  PASSED  
          sts_serial|  12|    100000|     100|0.03399278|  PASSED  
          sts_serial|  12|    100000|     100|0.20331620|  PASSED  
          sts_serial|  13|    100000|     100|0.32931990|  PASSED  
          sts_serial|  13|    100000|     100|0.56548087|  PASSED  
          sts_serial|  14|    100000|     100|0.89578460|  PASSED  
          sts_serial|  14|    100000|     100|0.26730315|  PASSED  
          sts_serial|  15|    100000|     100|0.93330449|  PASSED  
          sts_serial|  15|    100000|     100|0.98354430|  PASSED  
          sts_serial|  16|    100000|     100|0.89919591|  PASSED  
          sts_serial|  16|    100000|     100|0.49101210|  PASSED  
         rgb_bitdist|   1|    100000|     100|0.10557549|  PASSED  
         rgb_bitdist|   2|    100000|     100|0.49053193|  PASSED  
         rgb_bitdist|   3|    100000|     100|0.93107004|  PASSED  
         rgb_bitdist|   4|    100000|     100|0.55762324|  PASSED  
         rgb_bitdist|   5|    100000|     100|0.88239201|  PASSED  
         rgb_bitdist|   6|    100000|     100|0.27713422|  PASSED  
         rgb_bitdist|   7|    100000|     100|0.77245498|  PASSED  
         rgb_bitdist|   8|    100000|     100|0.89822762|  PASSED  
         rgb_bitdist|   9|    100000|     100|0.99727373|   WEAK   
         rgb_bitdist|  10|    100000|     100|0.32223307|  PASSED  
         rgb_bitdist|  11|    100000|     100|0.97288337|  PASSED  
         rgb_bitdist|  12|    100000|     100|0.93743658|  PASSED  
rgb_minimum_distance|   2|     10000|    1000|0.62533969|  PASSED  
rgb_minimum_distance|   3|     10000|    1000|0.82092795|  PASSED  
rgb_minimum_distance|   4|     10000|    1000|0.42047439|  PASSED  
rgb_minimum_distance|   5|     10000|    1000|0.04771354|  PASSED  
    rgb_permutations|   2|    100000|     100|0.05273059|  PASSED  
    rgb_permutations|   3|    100000|     100|0.41023943|  PASSED  
    rgb_permutations|   4|    100000|     100|0.78732277|  PASSED  
    rgb_permutations|   5|    100000|     100|0.28560341|  PASSED  
      rgb_lagged_sum|   0|   1000000|     100|0.04759587|  PASSED  
      rgb_lagged_sum|   1|   1000000|     100|0.25708472|  PASSED  
      rgb_lagged_sum|   2|   1000000|     100|0.78251732|  PASSED  
      rgb_lagged_sum|   3|   1000000|     100|0.87494710|  PASSED  
      rgb_lagged_sum|   4|   1000000|     100|0.85004277|  PASSED  
      rgb_lagged_sum|   5|   1000000|     100|0.54443223|  PASSED  
      rgb_lagged_sum|   6|   1000000|     100|0.76195523|  PASSED  
```

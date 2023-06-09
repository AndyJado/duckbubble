$# LS-DYNA Keyword file created by LS-PrePost(R) V4.7.7 - 17Feb2020
$# Created on Sep-27-2022 (13:20:31)
*PARAMETER
$#   prmr1      val1     prmr2      val2     prmr3      val3     prmr4      val4
$
$--- Simulation time 
R   dfailc       5.0
$
$--- Pre-force in beams / Pre-stress in solids (cross sect 74.9859 mm^2)
R bltForce      28.8
*CONTACT_AUTOMATIC_SINGLE_SURFACE
$#     cid                                                                 title
$#    ssid      msid     sstyp     mstyp    sboxid    mboxid       spr       mpr
         0         0         5         0         0         0         0         0
$#      fs        fd        dc        vc       vdc    penchk        bt        dt
       0.0       0.0       0.0       0.0       0.0         0       0.01.00000E20
$#     sfs       sfm       sst       mst      sfst      sfmt       fsf       vsf
       1.0       1.0       0.0       0.0       1.0       1.0       1.0       1.0
*END
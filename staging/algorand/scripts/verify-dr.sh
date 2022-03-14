#!/bin/bash
export SIGNATURES64=`node -e "console.log(Buffer.from('$2','hex').toString('base64'))"`
export GUARDIAN_KEYS='52A26Ce40F8CAa8D36155d37ef0D5D783fc614d2389A74E8FFa224aeAD0778c786163a7A2150768CB4459EA6482D4aE574305B239B4f2264239e7599072491bd66F63356090C11Aae8114F5372aBf12B51280eA1fd2B0A1c76Ae29a7d54dda68860A2bfFfa9Aa60CfF05e20E2CcAA784eE89A0A16C2057CBe42d59F8FCd86a1c5c4bA351bD251A5c5B05DF6A4B07fF9D5cE1A6ed58b6e9e7d6974d1baBEc087ec8306B84235D7b0478c61783C50F990bfC44cFc0C8C1035110a13fe788259A4148F871b52bAbcb1B58A2508A20A7198E131503ce26bBE119aA8c62b28390820f04ddA22AFe03be1c3bb10f4ba6CF94A01FD6e97387C34a1F36DE0f8341E9D409E06ec45b255a41fC2792209CB998A8287204D40996df9E54bA663B12DD23fbF4FbAC618Be140727986B3BBd079040E577aC50486d0F6930e160A5C75FD1203C63580D2F00309A9A85efFAf02564Fc183C0183A963869795913D3B6dBF3B24a1C7654672c69A23c351c0Cc52D7673c52DE99785741344662F5b2308a0'
export GKEYSBASE64=`node -e "console.log(Buffer.from('$GUARDIAN_KEYS',  'hex').toString('base64'))"`
export VAABODY=$3
export VAABODY64=`node -e "console.log(Buffer.from('$VAABODY',  'hex').toString('base64'))"`
rm verify.txn verify.stxn
goal app call --app-id $1 --from "$STATELESS_ADDR" --app-arg "str:verify" --app-arg "b64:$GKEYSBASE64" --app-arg "int:3" --noteb64 "$VAABODY64" -o verify.txn 
goal clerk sign --program vaa-verify.teal --argb64 "$SIGNATURES64" --infile verify.txn --outfile verify.stxn
goal clerk dryrun -t verify.stxn --dryrun-dump --outfile verify.dump
goal clerk dryrun-remote -D verify.dump --verbose



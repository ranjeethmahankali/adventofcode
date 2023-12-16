/*
--- Day 15: Lens Library ---

The newly-focused parabolic reflector dish is sending all of the collected light to a point on the side of yet
another mountain - the largest mountain on Lava Island. As you approach the mountain, you find that the light is
being collected by the wall of a large facility embedded in the mountainside.

You find a door under a large sign that says "Lava Production Facility" and next to a smaller sign that says
"Danger - Personal Protective Equipment required beyond this point".

As you step inside, you are immediately greeted by a somewhat panicked reindeer wearing goggles and a
loose-fitting hard hat. The reindeer leads you to a shelf of goggles and hard hats (you quickly find some that fit)
and then further into the facility. At one point, you pass a button with a faint snout mark and the label "PUSH
FOR HELP". No wonder you were loaded into that trebuchet so quickly!

You pass through a final set of doors surrounded with even more warning signs and into what must be the room
that collects all of the light from outside. As you admire the large assortment of lenses available to further focus
the light, the reindeer brings you a book titled "Initialization Manual".

"Hello!", the book cheerfully begins, apparently unaware of the concerned reindeer reading over your shoulder.
"This procedure will let you bring the Lava Production Facility online - all without burning or melting anything
unintended!"

"Before you begin, please be prepared to use the Holiday ASCII String Helper algorithm (appendix 1A)." You turn
to appendix 1A. The reindeer leans closer with interest.

The HASH algorithm is a way to turn any string of characters into a single number in the range 0 to 255. To run
the HASH algorithm on a string, start with a current value of 0. Then, for each character in the string starting from
the beginning:

* Determine the ASCII code for the current character of the string.
* Increase the current value by the ASCII code you just determined.
* Set the current value to itself multiplied by 17.
* Set the current value to the remainder of dividing itself by 256.

After following these steps for each character in the string in order, the current value is the output of the HASH
algorithm.

So, to find the result of running the HASH algorithm on the string HASH:

* The current value starts at 0.
* The first character is H; its ASCII code is 72.
* The current value increases to 72.
* The current value is multiplied by 17 to become 1224.
* The current value becomes 200 (the remainder of 1224 divided by 256).
* The next character is A; its ASCII code is 65.
* The current value increases to 265.
* The current value is multiplied by 17 to become 4505.
* The current value becomes 153 (the remainder of 4505 divided by 256).
* The next character is S; its ASCII code is 83.
* The current value increases to 236.
* The current value is multiplied by 17 to become 4012.
* The current value becomes 172 (the remainder of 4012 divided by 256).
* The next character is H; its ASCII code is 72.
* The current value increases to 244.
* The current value is multiplied by 17 to become 4148.
* The current value becomes 52 (the remainder of 4148 divided by 256).

So, the result of running the HASH algorithm on the string HASH is 52.

The initialization sequence (your puzzle input) is a comma-separated list of steps to start the Lava Production
Facility. Ignore newline characters when parsing the initialization sequence. To verify that your HASH algorithm
is working, the book offers the sum of the result of running the HASH algorithm on each step in the initialization
sequence.

For example:

rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7

This initialization sequence specifies 11 individual steps; the result of running the HASH algorithm on each of
the steps is as follows:

* rn=1 becomes 30.
* cm- becomes 253.
* qp=3 becomes 97.
* cm=2 becomes 47.
* qp- becomes 14.
* pc=4 becomes 180.
* ot=9 becomes 9.
* ab=5 becomes 197.
* pc- becomes 48.
* pc=6 becomes 214.
* ot=7 becomes 231.

In this example, the sum of these results is 1320. Unfortunately, the reindeer has stolen the page containing the
expected verification number and is currently running around the facility with it excitedly.

Run the HASH algorithm on each step in the initialization sequence. What is the sum of the results? (The
initialization sequence is one long line; be careful when copy-pasting it.)

--- Part Two ---

You convince the reindeer to bring you the page; the page confirms that your HASH algorithm is working.

The book goes on to describe a series of 256 boxes numbered 0 through 255. The boxes are arranged in a line
starting from the point where light enters the facility. The boxes have holes that allow light to pass from one
box to the next all the way down the line.

      +-----+  +-----+         +-----+
Light | Box |  | Box |   ...   | Box |
----------------------------------------->
      |  0  |  |  1  |   ...   | 255 |
      +-----+  +-----+         +-----+

Inside each box, there are several lens slots that will keep a lens correctly positioned to focus light passing
through the box. The side of each box has a panel that opens to allow you to insert or remove lenses as
necessary.

Along the wall running parallel to the boxes is a large library containing lenses organized by focal length ranging
from 1 through 9. The reindeer also brings you a small handheld label printer.

The book goes on to explain how to perform each step in the initialization sequence, a process it calls the
Holiday ASCII String Helper Manual Arrangement Procedure, or HASHMAP for short.

Each step begins with a sequence of letters that indicate the label of the lens on which the step operates. The
result of running the HASH algorithm on the label indicates the correct box for that step.

The label will be immediately followed by a character that indicates the operation to perform: either an equals
sign (=) or a dash (-).

If the operation character is a dash (-), go to the relevant box and remove the lens with the given label if it is
present in the box. Then, move any remaining lenses as far forward in the box as they can go without changing
their order, filling any space made by removing the indicated lens. (If no lens in that box has the given label,
nothing happens.)

If the operation character is an equals sign (=), it will be followed by a number indicating the focal length of the
lens that needs to go into the relevant box; be sure to use the label maker to mark the lens with the label given
in the beginning of the step so you can find it later. There are two possible situations:

* If there is already a lens in the box with the same label, replace the old lens with the new lens: remove the old
 lens and put the new lens in its place, not moving any other lenses in the box.
* If there is not already a lens in the box with the same label, add the lens to the box immediately behind any
 lenses already in the box. Don't move any of the other lenses when you do this. If there aren't any lenses in
 the box, the new lens goes all the way to the front of the box.

Here is the contents of every box after each step in the example initialization sequence above:

After "rn=1":
Box 0: [rn 1]

After "cm-":
Box 0: [rn 1]

After "qp=3":
Box 0: [rn 1]
Box 1: [qp 3]

After "cm=2":
Box 0: [rn 1] [cm 2]
Box 1: [qp 3]

After "qp-":
Box 0: [rn 1] [cm 2]

After "pc=4":
Box 0: [rn 1] [cm 2]
Box 3: [pc 4]

After "ot=9":
Box 0: [rn 1] [cm 2]
Box 3: [pc 4] [ot 9]

After "ab=5":
Box 0: [rn 1] [cm 2]
Box 3: [pc 4] [ot 9] [ab 5]

After "pc-":
Box 0: [rn 1] [cm 2]
Box 3: [ot 9] [ab 5]

After "pc=6":
Box 0: [rn 1] [cm 2]
Box 3: [ot 9] [ab 5] [pc 6]

After "ot=7":
Box 0: [rn 1] [cm 2]
Box 3: [ot 7] [ab 5] [pc 6]

All 256 boxes are always present; only the boxes that contain any lenses are shown here. Within each box,
lenses are listed from front to back; each lens is shown as its label and focal length in square brackets.

To confirm that all of the lenses are installed correctly, add up the focusing power of all of the lenses. The
focusing power of a single lens is the result of multiplying together:

* One plus the box number of the lens in question.
* The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
* The focal length of the lens.

At the end of the above example, the focusing power of each lens is as follows:

* rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
* cm: 1 (box 0) * 2 (second slot) * 2 (focal length) = 4
* ot: 4 (box 3) * 1 (first slot) * 7 (focal length) = 28
* ab: 4 (box 3) * 2 (second slot) * 5 (focal length) = 40
* pc: 4 (box 3) * 3 (third slot) * 6 (focal length) = 72

So, the above example ends up with a total focusing power of 145.

With the help of an over-enthusiastic reindeer in a hard hat, follow the initialization sequence. What is the
focusing power of the resulting lens configuration?

 */

#[cfg(test)]
mod test {

    fn part_1(input: &str) -> usize {
        input
            .trim()
            .split(&['\n', ','])
            .map(|word| {
                word.as_bytes()
                    .iter()
                    .fold(0usize, |hash, c| ((hash + (*c as usize)) * 17) % 256)
            })
            .sum()
    }

    fn part_2(input: &str) -> usize {
        input
            .trim()
            .split(&['\n', ','])
            .fold(vec![Vec::<(&str, usize)>::new(); 256], |mut boxes, step| {
                let (label, power) = step.split_once(&['-', '=']).unwrap();
                let b = boxes
                    .get_mut(
                        label
                            .as_bytes()
                            .iter()
                            .fold(0usize, |hash, c| ((hash + (*c as usize)) * 17) % 256),
                    )
                    .unwrap();
                if power.is_empty() {
                    b.retain(|&(l, _p)| l != label);
                } else {
                    let power: usize = power.parse().unwrap();
                    match (0..b.len()).find(|&i| b[i].0 == label) {
                        Some(i) => b[i] = (label, power),
                        None => b.push((label, power)),
                    }
                }
                boxes
            })
            .iter()
            .enumerate()
            .map(|(bi, b)| -> usize {
                (1 + bi)
                    * b.iter()
                        .enumerate()
                        .map(|(i, (_, power))| (i + 1) * power)
                        .sum::<usize>()
            })
            .sum()
    }

    #[test]
    fn t_part_1() {
        assert_eq!(part_1("HASH"), 52);
        assert_eq!(part_1(EXAMPLE), 1320);
        assert_eq!(part_1(INPUT), 505379);
    }

    #[test]
    fn t_part_2() {
        assert_eq!(part_2(EXAMPLE), 145);
        assert_eq!(part_2(INPUT), 145);
    }

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    const INPUT: &str ="\
jl-,dq-,cqn-,ds=5,slm-,fg=3,tp=3,rj-,svb=5,bbk=2,ntddjk-,fdmgq=1,hhk-,nrrqb=3,df=7,dls=3,\
ktf=1,gpvt=6,pc=5,cfgp-,xcj=5,grfsp=3,kq-,fxn-,vd=6,pz=6,nctp-,zd-,cts-,bpzv=3,nkvb-,cbnpmr-,\
jxpd-,pqq-,blz=7,xxdm-,bbj-,zlj=5,tgh-,zsnb=2,hjvh=3,kvr-,zz=9,xs=4,prxf-,qp=4,dlsvsf-,ch-,\
ppx=4,pbb=7,bvm-,bpzv=6,vznn-,nvpz-,xp=3,grxz=5,jmc=5,pv-,cbz=9,gx-,lsd=7,dz=1,bk-,lghttf=3,\
cp-,dg-,lmv-,sqf=3,nv-,rf=7,pr-,mzskhr-,svb=4,zdmj-,dn=1,lf-,phxfj-,xsmkc=9,fmc-,mgmns-,\
kq-,rj-,prhgp-,ngl=5,fxgl=6,rglq-,fz=8,vrf=9,xc=5,dn-,zdmj=1,lpj=8,bng-,ntj-,gzf-,nr-,\
tr-,vjr-,tfj=2,cl-,xsmkc-,ksdx=7,jfh=2,nl=6,gnct-,fnsp=1,nctp-,hpfcvc-,rxn-,zd-,pb-,ggx-,\
jqs-,bvm-,df-,fnrbm=8,nndr=5,lpj=2,qtr-,lmx=5,rs=5,zsd-,dhq-,xcqz-,nbz=5,blsct=3,blsct-,dn=3,\
rss-,sdb-,sz=5,sjk=7,qjlpvn-,grxz-,hc-,jhfpn=2,bqhj-,ph=4,rpnmvb-,szg-,crfk=6,tz=7,dq=6,xkc-,\
nrz=5,clc-,ks-,tzl-,jl=8,bbk=3,cfnp-,bxdg-,ndkh=9,llnz=6,qgcx=1,rpnmvb=5,ksdx=6,kjz-,xp-,rt=8,\
pqq=4,rxn=5,kp=1,bmgs=5,ts=1,nrgfj=7,qg=1,rs=7,bllm=3,pmh-,jlq=8,rldhlq=6,sqf=7,hqg=7,mrxj-,kcf=6,\
xb-,xh-,jv=2,tr-,jhfpn=1,lbd-,ds-,hhj=5,kk-,qjlpvn-,zfk=3,shl=3,vjr=3,bjcx-,sqf=4,fbpp-,\
rf=5,mdvcbf=5,nln-,gtg-,kqqj=8,ksdx=1,bfs-,zspr=3,qclqp-,gbql=5,lh=3,phxfj-,zlj-,hhj-,jcjqg-,xs-,\
kqqj=4,vxn-,jh=2,jh=5,tjs-,vvl=5,qclqp-,brdvpm-,zj-,lkc=6,gqzbl-,kg=3,nkvb-,hn-,lmh=1,bsms=4,\
gbql-,lsd-,ksbf=7,tjs=4,ztn-,xgx-,cvzr-,bqz=8,ghc=8,xlz=6,vs-,mktn-,ttrbn=8,xxhrd=6,mdr-,mz-,\
ds-,nrr=8,rf-,tgh=8,blz=7,fv-,lsd-,rxc=4,ndkh=7,xs=8,bbk=7,lvmx-,vdl-,gvl-,gn=1,xsg-,\
qg=1,vjj=6,kvr-,csbj-,rj=6,njfj=4,qtk-,sxm=2,rxc=3,dq-,dlx-,gnj-,shb-,bvngm-,khg-,lxcc-,\
gsls=5,jkb=6,bppr-,pff=9,bh=6,zj-,sf=7,bvngm=8,ngl=3,rf-,nv=5,hkd-,nv=6,nknpd=4,qhsz=4,mh-,\
mjqn-,xxdm-,rlk=8,hcq-,hjvh=7,fbd=3,szg-,mn=3,tv-,mxhfzh-,fxn-,szrtb=3,rgb=3,grm=3,bzx-,fbq=2,\
kjz-,gzf-,kbt-,shb-,xbk=9,xlq=8,lmv=6,bng=1,hcqcd-,xrnrc-,txkg=1,rb=2,nl=7,khcj=8,rhqg-,snn=5,\
mktn-,snn=1,jkb=2,zfgt=5,fbq=2,bq=5,zqt=8,tnb=8,bpzv-,lk-,hpfcvc-,ngt-,nkvb=2,dg=6,nrgfj-,prhgp-,\
tqvfl=7,dq-,rxn-,hcx-,xmkjd=1,nv-,rr-,hpq=9,bzx-,hrf-,vx-,bqz=9,bk-,cstn-,hgr=8,grxz=7,\
hgjq=8,txv=8,fvqs=5,gbql=3,xcj=7,qcmf=2,cts-,xmdx=5,cqn-,rxdmv-,nctp-,txkg-,bjp=2,xs-,lmv-,rk=7,\
rt=5,cz-,lkc=2,kzbg-,hbztt=4,lpc=1,nj=3,jlc-,vh-,ct-,dvtj-,ds=8,hjvh-,pr=1,fnn=6,bjcx=3,\
kh=9,ntj-,gbql=8,cfd-,lsm=3,gx=1,jqs=6,ds-,hhj-,hcq-,bqz-,zf=1,hpz=8,gmcgr-,xvpx=1,sg=5,\
tfh=8,sz-,pv-,sf-,cntbl=7,mdvcbf-,jfh-,xnnq=6,xgx-,gsls-,mgmns-,dbtjf-,zrn=4,tndl-,qxnr-,ntc-,\
bxlrz=3,blsct-,khg-,hrf=3,cdv-,zdmj-,gzf=7,bng-,jvks-,pbb=7,cbnpmr-,cq=6,bppr=2,pv=5,hnsrf=3,bf-,\
ctf=5,jph-,cz=7,vh=3,rlk-,xgx-,dvtj-,dhq=9,szg=4,blsct-,nrrqb=7,lc=2,ksbf-,lkc-,tnb-,gl-,\
cl=8,lq=7,mmgj=4,ckbt-,ckbt=1,hpz=2,pp-,fxllb=7,cq=9,rpp-,lqg-,lghttf-,gj-,hz-,xcqz=5,zvb=5,\
cvzr=7,slm-,lbr=2,tqvfl=4,qth-,xjq=1,gd-,phh=6,qgcx=9,lxcc-,pb-,gtt-,snn-,dnz-,xkc-,sjs-,\
tqvfl=5,dcr=9,nmv=1,zfgt=6,qcmf=8,sn-,xmkjd-,grm-,bd-,jkb-,qx-,qnz=5,nvpz=7,xmgxfq-,tzl=3,txkg=2,\
nq-,zskz=4,kp-,cp-,zspr=1,lq-,rmjzts-,dlx=8,jqmtfm-,bjp=1,fbpp-,rj-,dhqdz=8,lkh=7,lpj-,gl-,\
pmh=7,sg-,tc-,xjq-,rf-,zldr-,xkc=8,kqbrdm=5,bkm=4,rss-,fbq-,vs-,sz-,fxllb-,qcmf-,qlmm-,\
jmc-,xxhrd-,lbr=6,bmk=5,rss=3,rkf=5,rnh=3,bjcx-,lq-,shb=5,bzx-,ghc=8,rs-,jfn=8,mvmd-,dlsvsf-,\
pdbqhz-,bqz=9,vrf-,vs=6,kphps-,kk=6,nj-,pntd=3,bxlrz-,gn-,lkc=3,cz-,jkb=3,ttrbn=9,pl=2,sn-,\
hgjq-,tz=9,jvks=9,brdvpm-,gtt=1,tpn=9,khcj-,ghc-,jg-,mktn-,qhrhkh-,ch=1,njfj-,gn=8,zs-,fxn=8,\
rptz-,fvqs-,mxhfzh=3,xgr-,mgmns=3,pr-,fdmgq-,xcqz-,bxlrz=5,xkc=1,nrgfj-,ppp=3,lkh-,mh-,rg=5,gtvvdx=9,\
bbj=8,bjcx=3,qnz-,nrz=7,hqg-,mdvcbf-,lf-,tz-,zvbk=6,mvmd=5,slm=8,ql-,lz=1,xqr=4,cqn=2,bf=6,\
jlq=8,dnndg-,rt=1,lz-,xkhdmj-,fnsp-,sjn-,xsmkc=4,xsb-,lsm=6,fp=2,ntj-,fpq-,tgh-,tp=9,jcvsv-,\
rt-,pcg-,xmkjd-,pcg=9,ts=3,sjn-,gmtcl-,cq=3,ttrbn=5,bqz-,svb-,jlb=7,lsd=3,mjqn-,sbzld-,kxl=7,\
pz-,cbnpmr=2,hh-,vjj=4,xzm-,zb=7,qg-,hr=4,pn-,fz-,nndr-,pghp=5,lph-,hnsrf=4,bbk=2,qs=4,\
hkd=8,gb=8,hpq-,mvmd-,cbnpmr-,dn=9,zskz=2,rr=9,zdv-,sz-,mn=4,hcq-,cb=7,cfgp-,dhqdz=2,bppr-,\
ht=5,bk=9,xnnq-,cfnp-,sjn-,nmk-,dlsvsf=6,nmk-,bxlrz-,mgj-,kq-,sxm-,hkd-,xsg-,xql-,zskz=6,\
mn-,qhsz=2,dg=3,zlj=7,jfh-,jlq=1,rglq-,xh=9,hrf=7,sdb-,vc=1,gnct-,vznn-,vjj-,rmjzts-,hhk-,\
kzbg-,kxl=8,tnxvr-,ttrbn-,sr=3,mn=3,ds-,nl-,lmv=3,khcj=1,jv-,rs=2,fdmgq-,phh-,nj=4,fxn-,\
cntbl-,gtvvdx=7,shl-,rgk-,pghp=3,cbnpmr=3,ppp-,tnb-,gn-,brdvpm=1,kphps=6,tv-,pv-,khp-,txkg-,brdvpm-,\
gkk=6,jx=9,rv-,vvl=1,blsct=5,clc=7,qdqm=6,lmx-,lsd=3,rsf=4,bsms-,gqd-,xff-,bqhj-,tqvfl=1,dz=2,\
sq-,qtk-,fnn-,ppx=2,tc=8,bh=5,ndkh=4,sk-,gs-,dkn=9,gz-,xmdx=2,tt=7,bd=8,llcq=3,mdvcbf-,\
sht=1,gtt-,zn=3,gj=6,ht-,rg=2,nknpd=4,rd=9,sk-,lghttf=2,rdxc=1,cfgp=2,bq=2,rhqg-,ct=1,vs=9,\
nq=2,prxf=1,dg=7,fvqs=1,lxcc-,lph=2,crfk-,bh-,lxcc=9,vhdvm=8,ngt=6,xmdx=1,zkd=9,dlsvsf-,jhfpn-,tt-,\
zx=4,fdmgq-,ts-,qgcx=4,tndl-,mktn-,xsmkc-,gsls-,zlj=4,fc-,tt-,lmv=3,ntddjk=1,kvr-,pslcc-,xlq-,\
ktf=3,cl=7,tz-,gbql-,rmjzts-,tvb=6,qclqp-,pdbqhz-,vs=4,rt-,cq=4,hpfcvc=6,mtgj=6,bng-,npn-,sz=9,\
lbd-,ppx=3,ct-,qtk-,zl-,prxf=4,cdv-,jlq=2,npn=1,bvngm-,nj-,qg-,rpp-,kvr=8,mjqn-,lbd-,\
rxdmv-,nbsq-,ndkh-,ksbf-,jlq=2,lmv-,cj=6,gb-,xzsv=3,jl-,ntc=9,sbzld-,pl=3,hc=7,pzs-,xs=1,\
jcd=4,fdmgq-,cvzr=4,vc-,lf=7,pbb-,mgj-,ht=7,hn-,fpq-,cp=4,bqz=9,fnsp-,rptz=4,mxhfzh=6,dnz=1,\
vd-,bh=9,jv-,fnn=1,lkc-,rlk=6,xqr=8,bxlrz=4,fpq-,bmgs=5,lk=7,qhsz=5,cntbl-,qjlpvn-,tjs-,lph=2,\
nmv-,jl-,lz=4,hpfcvc-,qtg=1,lpc-,gz=9,pp-,sq-,jhd=5,rmjzts-,lp-,qs=6,szg=6,bmgs=9,rss-,\
zsd=4,kbt-,zsnb=5,crfk=5,lv-,pdbqhz=9,kzbg=9,shh-,rd-,lmv=2,sh-,jcvsv-,shl=3,fnsp=1,vjj-,mktn-,\
qhrhkh=3,mn=4,rss-,qfgdzd=4,lp=1,brdvpm=5,lk=5,vh-,vktbkh-,bh-,nl=5,sk-,jqmtfm=5,kjz-,gzf=4,xlz=3,\
zd-,zldr=8,zspr=8,nrgfj=3,zkd=6,npn=5,xkhdmj=4,xzz=7,pn-,jlb=6,vc-,grfsp-,fnsp=1,jfh-,zkd=4,shc-,\
gqd=2,gpx=7,qlmm=9,llnz=2,nl-,dls-,fdmgq-,gqd-,cntbl-,tclhnp-,dhqdz=4,nxkb=6,zj-,lh-,bfbc-,sk=8,\
jv=1,zvbk=4,ftvqt=5,gl-,lz-,ppj=3,zx-,dhqdz=3,tjs-,pc=5,lkc=4,mdvcbf-,fmc-,mtgj-,jv-,sk-,\
vtdld-,jmc-,kp=8,pntd-,dnndg-,fbq=6,jgggh-,lf=6,jvks=4,kzhd=6,pntd=4,vtdld=1,rss-,ksdx=7,lt=2,dc=7,\
jmc=8,hhj-,tjs=9,llnz=9,zg=1,nr=1,rhqg-,hh-,cnp=4,vjj-,hgjq-,qhsz=2,prhgp-,tqvfl=2,xd=2,llnz=1,\
zfk-,hl=9,gx=5,lph=8,ztn=4,vh-,fxn=5,zdmj=7,nknpd-,tpn-,cb=8,pbk-,rgb-,dbj=8,cp-,fxn=6,\
xzz=6,ksbf-,fxn=5,zvbk=2,cts-,pdbqhz=7,xb-,ntc=7,ts=1,vjr-,lqg-,xnnq-,rg=5,zz=6,hh-,xk=6,\
bkm=1,zspr=2,fbd=2,vx-,txv-,jl-,cbbj-,ksbf-,qjlpvn=2,ndkh=5,njfj=8,pd=9,bsms=8,jcvsv-,kphps=7,pj=7,\
bvsp=8,rr=3,ndkh-,xsmkc-,vqsjd=3,rsf-,dg-,xcqz=2,jl-,dlx=9,jmc-,ppj=7,ktf-,nv=2,mz-,cntbl-,\
lmv-,gtnhsv-,cntbl=9,rxn=6,zdmj-,qnz-,vxn-,nbz=4,ts-,mvmd-,gkk=9,zqt=1,fv=9,hz=9,nctp-,njfj-,\
nxkb-,mvmd=5,qv-,lz-,gb-,vznn=7,ppj-,hcq=9,nrrqb-,xql-,gx-,sg-,vqsjd=2,lpc=6,tlg=5,kjmb=4,\
tg=4,cz=5,cmb-,zb=3,gm=8,ndkh=5,fg=7,shh=2,dptc-,rv-,tjs=3,lt-,gb=8,fxgl=6,hhk-,blsct=4,\
xlq-,qdqm=6,pz=9,gpx-,nrr-,rr-,xzsv=6,tr=6,jlq-,zn-,ggx-,xbk=9,ph=2,tv=4,ql-,mgj=9,\
ksbf=6,zj=4,xcj-,xlq=1,jxpd=6,rsf=8,rn=4,qcmf-,rldhlq=6,bvm=8,tfh=6,gtnhsv-,hrf-,xh-,sq-,rldhlq=4,\
xc=3,jqs=6,xzsv-,qd=6,snbcc-,gpx-,srm=1,lk-,qlmm=7,ppx-,nrz=9,zsd-,zc=4,lz=6,rk-,bq=1,\
mrxj=3,kjz-,rxn=4,rglq-,jph=7,rxdmv=9,tfh=5,ckbt-,tj=2,srx=6,khp-,shh-,bppr-,lf-,bjp-,xnnq-,\
rxdmv=6,sxm=9,hz-,bml=4,fbpp=5,xff=3,lbd=5,prxf=3,cl-,gqzbl-,lmv=7,vd-,lghttf-,mgj-,gpvt-,tndl-,\
mrxj-,xp=6,nxkb-,txv-,pc-,jfh=7,jqmtfm=7,nv=5,rxdmv=6,kh=9,xxdm-,bsms=9,sjs=4,nt=5,pr=3,sjk-,\
dx-,qs-,sl-,mn=6,lv-,ppp-,jfh-,dfc=1,xzm-,tv-,ckbt=1,fnn-,dptc-,sjn=8,hl=9,lpc-,\
jcjqg-,ksdx-,tt=7,mjqn=3,jxpd-,fdmgq=8,jph-,xvpx=3,zlj=5,nndr-,xmkjd-,lkc-,vs=4,qth-,tnxvr-,zlj-,\
bf=3,gtnhsv=9,zlj-,jlq-,ctf-,svb-,rt=9,gsls=4,shp-,pghp-,jcd=6,nmk=2,rxdmv-,snn=3,npn-,bjp-,\
shc-,xgr-,gx-,tclhnp-,txv=2,gnct=8,kqm=5,bfs=4,mz=5,hl=5,lv=6,dncz-,rsf=7,jvks-,cp-,hbztt=3,\
mktn=9,dn=4,zqt-,jgggh-,rldhlq-,lpj=8,fbpp=8,sz-,cz=7,xzsv=9,srm=5,zskz=3,rsf-,jg=7,rdxc-,hhk-,\
bq=6,hgjq=2,bsms-,nvpz-,qx=9,gnj=6,sf-,pdbqhz-,sz=2,bkm-,cfnp=3,tpn-,dn=7,zldr=5,tt=4,zxfss-,\
npn-,pz-,kqqj=6,phh-,zb-,xhkhr-,rk=7,xff=6,qx-,rldhlq=5,mdn-,xzz=3,grfsp-,gmcgr=6,xk-,zsd-,\
qlmm-,dfc-,dls-,ph-,vktbkh-,tclhnp-,zspr-,qtk=8,tnxvr-,nctp=1,tv=1,ndkh-,df-,gx-,lk-,cntbl=8,\
zcb-,jlb-,cts-,zdmj-,nln=7,sh=3,ggx=5,tc-,phxfj=6,nbsq-,jv=8,pxf-,qd=2,mxhfzh=7,zfk-,jv=1,\
ngl-,bsms-,xxhrd=2,hhk-,snn-,zz=9,zj-,shc=5,hjvh-,slm-,zl-,sqf=1,tgh=2,zfk-,ks=2,nvpz-,\
xtt-,ct=1,hbztt-,fc-,sjk-,gz-,rlk=2,bzx=9,tv=9,gz=6,pz=9,zdmj-,jv=5,rnh-,mvmd-,kk=5,\
xjq=5,lsm-,ktf=5,grfsp-,xgx-,jcjqg=1,mdvcbf=9,qtg-,gzf-,qs-,kqm-,qd=8,xsmkc-,vdl-,kqm-,gkk-,\
jhfpn-,ksdx-,xh-,lsm-,mdn-,lk=4,jph-,ch-,gtt-,jkb-,zc-,hr-,qv=9,xd-,dlsvsf-,dlsvsf-,\
rj=4,hr-,bbj-,hhj-,nln=6,hl=7,rq=1,dn=2,jx-,sr-,jmc-,tnb=4,mvmd-,rhqg-,gb=2,mtgj-,\
rlk=1,cfgp-,zrn-,slm-,bml=5,nt-,hcx=6,gtnhsv=8,gvl-,cstn=6,gzf-,rkf=7,khcj=6,rj-,rs=3,lmv-,\
tclhnp-,fpq=9,tfh=3,bllm=2,gm-,qg=8,slm=9,gnct=7,jv=9,jph-,tpn-,gmcgr-,fxn=6,bjp-,pl=6,cfnp-,\
gbql=4,zldr-,cpqz=3,tfj-,zl=8,zsvn=7,xvpx-,xql-,xtt-,blsct-,ggx=4,mtgj-,ph-,kp-,rh-,hz-,\
xp-,khp=5,hz-,xxdm-,vvl=8,xcj-,tj-,fc=5,dlx-,zj=6,zsvn-,dhq-,bml-,tlg=7,hrf-,vdl-,\
mdvcbf=5,gsls-,nmv-,njfj-,ktf=7,qcmf-,lz-,dhq-,xgx=9,ckbt=3,vjl-,kxl-,tzl=9,tjs-,phxfj=5,xzz-,\
rh=8,vh-,gsls=1,nndr-,qxnr=6,nlr-,gqd-,jkb=8,hbztt-,fdmgq=8,rglq-,lsd=4,fnsp=1,rs-,zj-,bzx=2,\
zspr-,gnj=5,ph-,ttrbn-,ndkh-,mktn-,dn=5,cfd=1,lsm-,gbql-,rgb-,ftvqt-,bqz-,ppp-,zn=8,dhq-,\
xcqz-,nmk=8,pz=1,gqzbl-,hpfcvc=1,bmk=7,nbzvg=4,xlq=9,rldhlq-,nrz=9,fbq-,htxq-,jfc-,rss-,llcq=4,xlq-,\
fnn-,grxz-,zrn=4,lghttf=9,nt-,kh=1,dz-,nln=2,dz=5,ct-,kg=7,bzx=8,vktbkh-,pdbqhz-,jhfpn-,lf-,\
slm=3,fmc-,cvzr-,jqmtfm-,fbq-,rnh-,xvpx=3,ppx-,ngt-,bqhj=7,cbgpbl=4,zd-,cvzr-,ztgc-,zkd=2,sf=1,\
jfc=1,ftlk-,npn=9,tqvfl-,zsnb-,mz-,tr-,dfc=3,kqbrdm=6,vs=8,fg-,qdqm=8,kjmb-,snbcc=7,vvl=2,sr-,\
jfc=5,cmb-,sdb=9,jx-,xxdm=2,fbpp=9,txv-,ztn=1,nlr=2,fv-,prhgp=9,kvr=5,pb-,qz=3,gnct-,rr=3,\
prhgp-,lktp=5,zx=9,rs-,zfgt=5,jcvsv=2,rx=8,bjp=5,grxz=8,crfk-,rs-,qth=5,xlz=4,cz-,xjq=2,blsct-,\
jkb-,qgcx-,fbq-,tp-,rh-,khcj=8,xmgxfq=2,lmh-,dvtj=9,ngl-,rdxc=2,sbzld-,shl=7,dlx-,nlr-,zc-,\
vdl-,lghttf=1,hn-,vvl-,dcr-,tgh=9,dfc-,kxl=8,vktbkh=5,gkk=4,xlz-,kh-,tfj=2,ggx=3,bllm-,xmdx=4,\
rf-,zdv=7,txv=8,bd-,qs-,tnxvr=5,gmtcl=3,cz-,df=6,fbq-,pc=5,ntc-,khp=8,vqsjd=3,hr=1,lmx-,\
qcmf-,xqp-,bvsp=6,rdxc=1,ntc-,jcjqg=5,jfh-,zcb-,khcj-,cntbl-,rxdmv-,lkh-,hh=7,lpj=8,txkg=4,jlc-,\
rgb-,zfgt-,cmb=2,tndl=2,bbj-,nq=8,jlb-,phxfj-,mdn=1,zspr-,dls=2,pr-,zsvn-,ksbf=1,tj=5,jkb-,\
sht=2,jhfpn=8,fp-,rnh=4,rsf-,fbpp=8,cbnpmr=4,sqf-,gz=1,bml-,gbql=2,mjqn-,bxlrz-,gzf-,tnxvr-,nknpd=7,\
df-,nrz-,nt-,prhgp-,lkh=5,zb=3,bbk-,bllm-,dx=7,hgr=7,xlz=5,rd=8,pntd=5,zl=6,bsms-,cvzr=2,\
vx-,xxhrd=7,lvmx=2,vqsjd-,xsb=4,rhqg-,pp=3,fxgl-,zx-,xqp=5,shl-,fxn=8,tgh-,xkc=7,sbzld-,bzx-,\
zmpqd=1,bmgs=2,jgggh-,ksdx=2,pp=9,ghc=7,shc=8,jl=7,cpqz=2,pl-,ldfnr=5,mgmns-,bbk=8,ngl=8,hjvh=5,fv-,\
ttrbn-,zspr-,grxz=9,ftlk=3,qhrhkh=2,kk-,vjj-,ts=6,kqm-,rss=2,pv-,nr=9,xsg=5,qclqp-,grm=8,qfgdzd=6,\
xsg=7,xxdm-,rptz=7,jfh=7,fxgl=2,lbd-,zs=7,jhfpn=1,gkk=6,jh-,pd-,jcjqg-,cp-,mn-,vdl=5,rdxc-,\
nknpd=1,bkm=6,xjq-,rptz=5,hc-,pb=3,kqbrdm-,gx=5,llcq-,gmtcl=5,tr-,vrf-,tndl-,jhfpn=4,nkvb-,nl-,\
ztn-,bl-,zspr=3,vqsjd=9,fxn=3,grfsp-,gpvt-,rpnmvb=9,jcvsv=8,pb=1,gqzbl=4,mh=1,vjj-,lmh=2,kcf-,bmgs=5,\
glcb-,xh-,tz-,rb=2,sg-,xhkhr-,lp-,xcqz-,svb-,bvngm-,nmk=3,cts-,fvqs=6,mgmns=5,tzl-,cj=7,\
xrnrc=8,ksbf=9,jqmtfm=4,sg=5,lv=2,cq-,rglq-,mgj=3,sq-,qd=5,rg-,rdxc=3,lmv=9,tp=1,gx=5,ts-,\
gbql-,tvb-,hcqcd-,mz-,qcmf-,cfgp-,jfc=2,bml-,pslcc-,srx=3,vvl=3,pz=4,mh=6,slm=9,hkd=7,sz=4,\
df-,jxpd-,bvm-,tvb=9,lpj-,xcj-,gn=7,mdn=3,ztgc=3,hc=1,rxdmv=6,mrxj-,zvbk-,pghp-,xbk-,qcmf-,\
jlq=8,tfj-,nv-,cfnp-,rq=9,slm-,lktp=1,zb-,jcvsv=8,cmjl=9,bppr-,nrgfj-,nln-,nkvb-,gpvt-,qdqm-,\
tlg=4,gnj=4,dx=7,gzf=6,gmcgr-,vxn-,lktp-,qcmf-,tvb=4,zsd=3,gqd-,ggx=3,kk=8,qz-,kh-,zx-,\
ztgc=7,hz-,xgr=9,zxfss=5,khcj=2,lxcc-,xzsv-,kzbg-,tnxvr-,dbj-,vx-,kzhd=8,xjq-,fxllb-,jx=9,bsms-,\
vrf=7,dlsvsf-,ppx=8,dls=8,xs-,gz=8,zlj=1,ckbt-,qgcx=9,kg=5,snn-,zspr-,nmk-,bllm=6,cl=1,kbt-,\
bd=3,cxz-,cts=1,sg=8,cp-,hrf-,lt-,zd-,qclqp=4,vdl=9,zn-,kxl=8,qjlpvn-,gtg-,nlr-,dlx-,\
jhfpn=5,bxlrz=8,cntbl-,zldr=1,rr=6,cl-,nvpz=4,qfgdzd-,rptz-,hpfcvc-,dlsvsf-,phh-,lmv-,fz-,lkh-,dn=2,\
zsd=8,lbr=3,rmjzts-,xjq=2,jxpd=5,lf=8,tfj-,lk-,rldhlq-,zsd=5,rmjzts=9,cdv=6,ghc-,bppr=5,fxgl-,xs=4,\
rf=9,qv=9,lsd=9,cj-,xqp=3,jhd-,dg=8,kjz-,gm-,grm-,kxl=7,snn=7,dbtjf=5,xcj-,shl=4,szrtb=7,\
jvks=3,rlk=6,gb-,gnj-,rt-,gs-,xgx=8,rdxc=8,rr=7,xqp=1,nq=6,qx-,ntddjk-,xcj-,gsls-,dx=6,\
pzs=1,sg-,xbk-,df=8,szrtb-,znz-,sr=1,cb=9,vtdld-,slm=9,tjs=3,qjlpvn-,qnz-,cnp-,dlx-,fg-,\
pmh-,bvm=3,pb=9,clc=4,zkd-,cb=9,rdxc-,zb=6,tpn-,rr-,nl-,ppp=6,jfc-,jph=1,lph=4,xql-,\
lph-,bxdg=9,ql=6,sq-,mvmd=6,lq-,txv=2,nxkb=5,xql-,sk-,dn-,jcvsv=9,pc=8,bvngm-,dx-,fnsp=7,\
qxnr-,pslcc-,zlj-,ztn-,tqvfl=8,xg-,shb=5,nknpd-,xg-,khp-,cl=1,kqqj=1,ckbt-,rkf=4,fxllb=6,rb=7,\
jv=2,bxdg=9,nr-,mz-,jcvsv-,mh=7,bq=6,xtt=3,qhsz-,cbgpbl=4,ndkh-,nr-,rgk-,cz-,pn=1,shp=3,\
rkf-,jx-,sht=4,jfc=7,rj=3,txv-,nv-,zn-,rptz-,ctf=8,bjp=9,rxdmv-,tp-,lbd=7,tnb=8,jl-,\
mrxj=5,lpc=9,jv=8,hpz=7,zvb-,kzbg=9,xg=8,nl-,pbk-,lk-,qg=7,vh-,jlc=3,ntddjk-,ntddjk-,mktn=6,\
csbj=4,rh=6,rpp=6,xtt-,cstn-,lkc=2,nrr=3,snbcc=4,nlr-,lpc-,vqsjd=8,bvsp-,jxpd-,gtg=1,zfk-,cj=4,\
hcqcd-,qth=1,mqz=2,mvmd-,cz-,lktp-,vxn=8,mgmns=3,khcj=9,nrrqb-,gqd-,cfgp-,nkvb=1,kqbrdm-,tg=1,vdl-,\
gj=3,mn-,tfh-,vznn=6,jfh=4,khg=8,grfsp-,kzbg-,zj-,sl-,cmjl-,cqn-,vqsjd=6,npn=1,ztn=2,qg-,\
nrz-,fnn=2,szrtb-,zvbk=4,cmjl-,npn=7,gvl=9,vhdvm-,fnn=7,xh=3,vd-,qfgdzd-,gbql=5,ntj=8,bfs-,bzx-,\
ppx=2,tlg=8,dg-,jfh=6,zg-,rmjzts-,gl=4,vznn-,cj=2,cbz=8,pqq-,dvtj-,tp-,bppr-,hc=4,pdbqhz=6,\
xqp-,bpzv-,jcjqg=1,hh=8,gm=3,bvngm-,jqs=2,ngl=1,gzf-,mq=2,bd=6,gpvt=1,kqbrdm=9,cmjl-,xrnrc=4,ts=6,\
bml=8,zrn-,srm=7,zfgt-,cstn-,zvbk-,hh-,bbj=5,jkb=4,qgcx-,rh-,xh-,fdmgq-,pbk=9,ghc-,xgr=8,\
ctf=7,ts-,pv-,xmgxfq=5,svb=1,cfd=7,gqzbl-,zdmj-,mgj-,lghttf-,jhfpn-,rss=1,vktbkh=1,rnh-,jlc-,gn=1,\
npn=1,lvmx=1,bmgs=7,ztgc-,cfnp-,qtg=8,glcb-,nj=1,rh-,mn=2,lghttf=8,qz=4,grm=4,fxllb=7,sf=4,szg=6,\
qhsz-,mq-,zfk-,mrxj=8,zskz-,csbj=5,nxkb=6,rkf-,xsmkc-,jxpd=6,zx-,qtr-,gm-,ts=1,dcr-,dnz-,\
lv-,gm=3,gtnhsv=3,pp-,xxhrd-,zd=7,bxdg-,cbnpmr-,xql-,jxpd-,lkh-,qtg=5,lt-,gs-,jgggh-,vjj-,\
ggc=3,ql-,cfnp=2,pxf-,gsls-,ts-,sl=3,fbq-,lktp-,rxdmv=7,tg=3,hc=4,jlb=7,zlj=5,gmcgr-,gsls=7,\
jl-,bml-,zc-,rxdmv-,rpnmvb=4,vd-,pv-,mtgj-,shc=6,xp=4,tnxvr-,cp=3,gpx-,zn=1,jhfpn=7,mktn=9,\
ks=5,gj=5,grfsp-,kzbg=7,brdvpm=8,jfn-,sf-,jlb-,pdbqhz=4,ht-,tr-,nrgfj=3,nrr=8,zf=9,bjcx=2,qlmm-,\
qgcx-,lktp-,tr-,fmc-,rldhlq-,rxdmv-,lxcc=4,ct=2,qhsz=6,gvl=8,tvb=3,zlj-,cpqz=7,cpjn-,tj-,brdvpm=8,\
zkd=5,pghp-,qcnv=8,ksbf-,pc=4,tpn=2,jkb=8,ppx-,ktf=6,lz=9,ppx-,jg-,nlr-,cfgp=1,hn-,rh-,\
fc=3,gtnhsv-,zldr=3,rnh-,tp-,mdr=2,rr-,xsg-,xql-,jfn=1,fg=5,mtgj-,ppp-,mh-,pp-,ztgc-,\
fg-,xvpx=2,jl-,bqhj-,gtnhsv=4,shh=2,xbk-,gpx=2,xnnq-,bkm=9,jmc=7,zdv=2,xd=4,ch-,rldhlq=1,ppj-,\
lt=7,pz=5,szg-,bqhj=3,hhpk-,bkm=3,gnj-,mgj=4,hkd=4,kbt=9,zldr-,bfbc-,csbj=9,zcb-,cmjl-,bmk=8,\
cnp=7,rgb=6,mvmd-,mmgj-,ph=7,mdn=5,npn=5,nlr-,kxl=7,fmc=2,fbq=1,cbbj=1,xzm-,vxn-,jph-,dhqdz=4,\
zz-,sg-,kh-,nbsq=1,bqhj-,jlb-,vhdvm=1,rpp-,zb-,vh=1,szrtb=2,nxkb-,zqt=6,prhgp-,cvzr=5,bsms=7,\
cstn=3,qs-,kzbg=9,cmjl=1,pbk-,csbj-,pb=9,xgx=3,lsd=4,xb-,zg=2,lvmx-,fvqs-,hl-,fmc-,cxz-,\
xsmkc=9,zxfss-,bxlrz=5,pqq=6,bfbc=9,tc=9,zsnb=7,jg=6,sl=4,xrnrc-,shb=2,zvb-,hcq=2,qxnr-,dbtjf=1,zmpqd=2,\
rdxc=2,txv=2,zg=5,zz=5,pp-,clc=5,hjvh-,ctf=3,rldhlq-,xbk-,zl-,qhrhkh-,ztgc-,qcnv-,jqmtfm-,zdmj-,\
kjz-,cnp-,mktn=9,tnb-,bjcx-,hgjq-,vjr=8,zvbk=6,bfs=4,rd=1,fpq-,tvb-,zd=6,dlx=5,qhsz=2,ngt=9,\
pff=9,hkd-,xmdx-,hkd=4,bjp=1,jh-,ch-,cvzr-,qs-,cfd-,tt-,dbtjf-,bvngm-,zsd-,nndr=9,gkk-,\
rn-,tjs-,xxdm=9,cfgp=4,blz-,nbzvg-,gl=2,zqt-,rb-,zlj=7,fz-,rkf-,tnxvr=5,hpfcvc-,nr-,snbcc-,\
rv=6,gpvt-,ggc=5,hh=3,gthg=7,kq-,shp=5,fpq=7,tt-,qjlpvn-,fpq=2,bmk=3,rgb=3,ppx-,xqr=7,qx=2,\
ql-,rglq-,cntbl-,cfnp-,cstn=3,khcj=7,pn=5,gpvt-,pbk=5,njfj-,rgb-,xk=4,zfk-,tgh=9,pqq-,cz-,\
cntbl-,zvb-,mdn=6,llnz=6,vrf-,xlz-,zfgt-,pxf-,tclhnp=2,szg-,ht-,vs=7,vhdvm-,pxf-,lbr=5,hc-,\
nq-,nrrqb-,qz-,cqn=4,ts-,kvr-,mdn-,vh-,kphps=1,bfbc-,lpj=5,mdn=1,lp=6,sr=8,zldr-,khg-,\
jh-,xqp-,qhsz=2,qx=7,mn=9,jgggh-,fbd-,lghttf=6,phxfj=8,mmgj=3,fnn=2,fnn-,bvngm=3,mzskhr-,zldr-,lbr=8,\
blz=1,lpc=4,lt-,pd=7,hv-,ksbf-,vqsjd-,pzs=2,shb-,gd-,tj=3,bf=2,qxnr-,gmtcl=7,npn-,pbb-,\
zdmj=8,pr=5,sjn-,gm=9,pntd-,gtt=2,qdqm-,ckbt-,lkc=9,tv=3,zskz-,xlq-,mvmd=2,jfh=7,mdvcbf-,pqq=4,\
gtt-,nndr=1,lvmx=5,sr=4,lqg-,pff=4,shb=2,qcnv-,pl-,nctp=1,kp=9,pmh-,zdv=1,nmk-,nr-,cpjn-,\
lkh=7,tg-,lq-,bjcx=5,cfgp-,rd=3,xnnq-,vrf=8,xlz=1,qz=3,bq=5,zz-,cts-,bjp=6,nxkb-,gm=6,\
jcjqg-,vxn=3,tp-,mqz=9,gqzbl=3,hv=7,mxhfzh-,mdn=7,qp-,xg=6,xg-,rxn=8,fdmgq=5,jlc=7,brdvpm-,jcd=7,\
cbbj-,lp=1,xff-,hhpk-,jhd-,cfgp-,xrnrc=8,nmv-,bsms=6,jx-,grfsp-,jfh=1,ctf-,ktf=4,brdvpm=6,jph-,\
xk-,bsms=3,ftlk=9,ctf-,fpq-,khp=9,lsd=2,lk=1,szg-,lc=7,nrr-,hkd=7,gtg-,tj=3,fxgl=1,nmk=2,\
nxkb=2,qth-,vs=3,kp-,cbnpmr-,qtk=5,zsd-,hnsrf=4,ksbf-,rptz-,rn-,ql-,cpjn-,sl-,vd=7,jlc-,\
dlx-,vqsjd-,jlq=1,zfk-,prxf-,zfgt=3,zvbk=1,rxn-,pb-,gsls-,zsvn=8,ftvqt=8,cfgp-,cpqz=9,nctp-,lmx=8,\
kxl-,tzl-,bqhj=4,qs-,bfs-,fvqs=5,fxllb=3,tqvfl-,cj-,rpnmvb=9,bmgs-,mqz=2,xgr-,srm-,tfj-,qhrhkh=1,\
dnndg-,hz-,lqg-,fmc-,xh=6,bq-,xbk=3,nl=2,cts=2,xsb-,ch-,dhqdz-,phxfj-,lmx=5,rd-,rt-,\
qp=3,cj=4,xc-,rgb=3,rn=7,bqz-,zj-,hn=8,kphps=3,sjs-,mjqn-,lghttf=9,gmcgr=9,nknpd-,sn-,nbzvg=4,\
qz=7,tclhnp=7,mgj-,hbztt=6,zvbk-,shl-,kqm-,bzx-,tndl-,rh=7,dx=9,xlq-,lh-,zb-,mzskhr=5,mvmd-,\
jcd-,hl=6,csbj=1,vs-,dvtj-,tg-,rgb=4,bllm=9,xgx-,lq=4,xmdx-,gtnhsv=4,rg=6,khg-,crfk-,mn=3,\
cp-,hpz-,gn-,shp=9,ql=3,pdbqhz-,pc=6,ftlk=7,szrtb-,tqvfl-,dlx-,qxnr-,cqn-,gs=8,nvpz=8,fnsp-,\
tz-,hrf-,qcmf-,pff=9,tg-,rxc-,tp=6,jlc=4,xmkjd=9,pc=8,sg-,gbql-,xmdx-,zc=5,dhqdz-,ntj-,\
bvsp=8,bf-,ds-,pxf-,gthg-,jgggh=2,dcr=5,dlx-,kphps=2,nrrqb=6,rx=3,nbzvg=2,nv-,xzz=9,zx-,kqbrdm-,\
vd-,rx=6,bppr=2,tj-,cdv-,dn=9,qd-,cz=5,ndkh=8,qjlpvn=5,zg=1,bq-,fc=4,cmjl=8,mdvcbf-,dg=2,\
jqs=6,dcr-,ftlk-,bmgs=2,cqn-,jh=2,xxhrd-,gmtcl-,qtr-,bbk-,fbq-,gb-,gtnhsv=8,xql-,hhj=7,mxhfzh-,\
ctf=8,pr=4,cfnp=8,hhj=7,mjqn-,zd=4,zx-,jqs=2,rs-,bng=3,kjmb=2,cpjn-,lsd=2,jg-,zdmj=9,gmcgr-,\
pp-,qcmf-,rh-,kzbg=8,bvngm-,dbj=1,vs-,bh=7,fxllb=7,qg-,fbd-,jcd=7,pghp=4,bppr-,lmv=8,mvmd-,\
kqbrdm=5,mjqn-,lph-,sbzld-,lkh-,lpj=3,pbk-,zsd=5,gnj-,cq-,gb-,zqt-,lbr-,jfh=8,kqm-,ph=4,\
cj-,qxnr-,rgk-,bh-,bppr-,zsd=9,qv-,sjk-,xs-,shl=6,jcd-,dbj-,zlj-,rpp-,jlc-,mdn-,\
gtg=1,bjp=5,bllm-,lh-,bvngm=8,prxf-,mjqn=2,zn-,hpfcvc=4,fbq-,ghc-,rss=9,hjvh-,mq-,rj=8,jx-,\
rdxc-,vtdld-,qhrhkh-,fg=3,nl=2,cvzr=4,hcx=7,rbt=8,gx=2,lvmx-,nrr=8,hqg=1,zspr-,xqr-,mxhfzh-,pntd=7,\
vjj=7,fv-,hkd=9,nmk-,tnxvr=7,gn-,rxn=8,ttrbn-,kqbrdm-,tfj=7,khg=1,xzsv=5,tvb-,prxf=6,jhfpn=8,gmtcl-,\
ngt-,ds-,xh=5,ts=5,gn-,hhk=5,rkf=5,jhfpn=4,jcd=8,lghttf-,gs=1,kbt=8,bf=4,pqq-,pxf-,sjs=1,\
jqmtfm=8,xqp-,jmc-,jl=5,ntddjk-,xd-,nv=8,cfnp-,nkvb=9,mktn-,rg-,bml=1,cts=5,df-,jcd=2,kvr=9,\
jfh=2,gb=4,gqd=3,xhkhr=1,rpp-,khp-,ttrbn-,zn=1,llcq-,xql-,cz=3,qs-,jqs-,hgjq=1,pp-,ntj-,\
qtr-,ztn-,zx-,tg=5,xd=1,xk=4,ghc-,shh-,sjk-,kk-,xql-,vhdvm-,zrn-,ppp-,ftvqt-,xb-,\
kq-,xqr=5,hkd=4,xlq=2,gz-,gzf-,dhq=5,sg-,nvpz-,cqn=2,ntc-,llcq-,txv=5,gsls=6,xcqz-,mz-,\
bqz-,hhk=7,vhdvm=8,hpq=3,tnb-,qp=5,fg-,vjl-,gmcgr-,jcjqg-,tjs-,lt=9,fbpp-,qs-,lph-,tndl=7,\
qz-,jqmtfm=6,ntc=6,nbz-,pc-,zsvn-,gpvt-,llcq-,mtgj-,bbj-,kvr-,zfk=1,zskz=8,tc=7,pd-,znz-,\
bk-,nl-,pqq=5,zcb=5,gthg-,nbzvg=9,cmjl=3,zc=7,vx-,grfsp=9,mmgj-,tfh-,mdvcbf=1,jqs=9,hc-,jlq-,\
kq=2,dkn-,pv=8,ks-,bvsp-,gkk-,jh-,jh=8,grxz=3,bfs=6,hcqcd=9,dbtjf-,jh=4,rn-,grxz-,jcjqg=2,\
mn=2,nr=7,mktn-,srm=6,rbt-,qclqp=8,lc=2,rx=1,gs=4,sjk=7,jfh=2,prhgp-,sbzld-,gvl-,sl-,pmh=9,\
hz-,hcq=7,mn=1,mdvcbf-,mdvcbf=3,cstn-,rq=1,lkh-,khp-,sjk=8,dg=6,jlc-,rxn-,nbsq=6,cbbj=3,ftvqt=1,\
pb-,lpj-,hr=5,ksdx-,qxnr-,hrf-,srm-,sq-,dq=2,sn=2,qnz-,lf=1,pbb=4,grm-,nknpd-,jfh=2,\
xzsv-,gnct=2,cmb=9,lghttf-,lmx-,gd-,xh-,mgmns-,lmv-,hhj-,xxhrd=4,ktf-,lsm=5,cb=9,rxdmv-,ngl-,\
ql-,gl=6,jcjqg=3,qp=3,ht=8,tj-,qclqp=4,lbr=8,lxcc=1,tvb-,gtt-,kqqj=7,sqf-,xlq=3,dq-,ppx-,\
bllm=8,qx-,hc-,cnp=2,rs-,bh=5,zlj-,shc=2,qhrhkh=9,dlsvsf-,vd=8,mz-,zlj=2,rhqg-,vc-,qp=3,\
kjz-,bvm=5,ztgc-,zz=2,rd-,pslcc=8,shc-,nr=5,gmtcl=5,qth=5,gn=1,zqt-,bvsp-,ds-,fnrbm-,pj=9,\
mz=4,cpqz-,hhpk=7,mz=6,lsd-,xmgxfq-,bf-,vx-,jlq=1,rldhlq-,xxdm-,bkm-,dz=2,cstn=4,tzl=7,nbzvg-,\
zxfss-,sht-,dz-,cntbl=6,gqd-,sn=3,rlk=2,gd=4,lsm-,fbq=1,bvm-,vznn=6,ggc=9,xqp-,tclhnp=5,pl-,\
xkhdmj-,dg=8,fbpp-,lbd-,gnj-,nbz=9,hpfcvc=1,lmv-,qjlpvn=5,hrf-,htxq-,xnnq-,nctp=4,kp=9,pntd=3,ds=4,\
xmdx=5,pslcc=4,mmgj-,jcjqg=4,bqhj=1,kphps=8,hcqcd-,cnp=7,kzhd-,hqg=7,xjq-,llcq-,sqf=4,nrr-,nmv-,mq=8,\
nrrqb=5,pqq-,lmx-,prhgp=1,pr=5,xlz-,nl-,cfd=4,xh=1,rxdmv=5,xd-,bmgs=9,lghttf-,sdb-,sz-,sr-,\
qhrhkh=2,ch=2,mz=9,gtvvdx=7,zf-,sl-,xzz-,phxfj-,dnz-,xsg-,gx-,bq=8,gqzbl=5,vs=9,nbz=8,cbz-,\
rb-,hqg-,bsms-,ktf=9,hhj=3,prhgp-,ct-,kq-,cpqz=8,pbb-,cnp-,sk-,mxhfzh=8,hpfcvc=7,nknpd=9,pj=2,\
jcd-,rg=8,mh=7,hqg=2,rb=2,xp-,rq-,brdvpm=7,sk=6,tnxvr-,zlj=5,qtk-,gthg=8,bk=8,bsms-,xg=1,\
ckbt=6,sg-,shl=5,mdr-,xjq=1,pdbqhz=4,hcx-,dncz-,mdr=5,hc-,txv=8,fxn-,ql-,fvqs-,pb=7,vxn=1,\
zlj-,xqr=5,gqd=9,shh-,jcvsv=3,prhgp-,pcg=3,cmb=4,gj=3,zspr=7,xkhdmj-,glcb-,dnndg-,xzz-,vxn=9,kjmb-,\
jlq=4,kzhd=5,hcqcd=7,ztgc-,qjlpvn-,tlg-,gl-,pff-,cbnpmr-,clc-,mz-,kjz-,nrrqb=6,jqmtfm=7,hrf-,bsms=4,\
nrz=5,nj=6,rpnmvb-,qclqp=8,gj-,vc-,ndkh-,zskz=1,rj-,tfj-,bk-,sht-,gn-,jmc-,hh-,lktp=8,\
rgk=6,xk-,rmjzts=9,hh-,svb-,pxf=4,rt-,rt-,xsmkc=4,xnnq-,rss-,vjl=8,pv-,kphps-,gmtcl-,jqmtfm=2,\
gtvvdx=3,sf=8,gkk-,vqsjd-,rptz-,ql=6,dhqdz=8,blz-,gpx-,zxfss=7,rhqg=3,bvm-,zsvn=5,nxkb-,mmgj-,gtvvdx=2,\
sdb-,pghp=6,rnh=9,zrn-,cj-,xql-,pdbqhz=7,mh-,sxm=9,tp=9,clc-,dlsvsf=6,llcq-,tlg=9,slm=4,pslcc-,\
sl-,njfj-,sr-,gd=9,zvbk=9,bfs=8,qfgdzd-,cts-,lq-,hhj-,zspr-,tclhnp=2,zl=5,rhqg-,brdvpm-,cts=1,\
zfgt-,zsvn=8,xkhdmj-,xvpx-,tv=5,shh=9,ct=5,ztgc=3,xbk=8,lpc=6,ht-,rxdmv=4,dn-,jfc=9,rgb-,crfk-,\
zkd-,zkd-,gl-,qp=9,qjlpvn=6,mktn=3,fz=9,ksdx-,slm=7,jmc-,jfh-,lghttf=3,cxz=2,gthg=4,vznn-,hqg-,\
xxhrd-,fpq=8,gb=5,nj=1,jhd-,bfs=1,bjcx-,bk-,zz-,xd-,qx=7,pqq-,sn=9,tnb=2,xqr=7,dz-,\
bfbc=4,hpz-,cdv-,qz-,fxn-,hnsrf-,rldhlq-,sdb-,xb-,jfn=3,dls-,npn=9,kk=3,vjr=1,qhrhkh-,pz-,\
bvsp=3,kqm=9,cxz-,lmv=3,dlsvsf-,lh-,tc-,qtg=1,ppj=8,qz-,zsnb=4,xb=7,kqm-,kzbg-,cq=4,bf-,\
cbbj=5,zlj-,bng=9,tclhnp-,hpfcvc=5,fc=8,blz=7,ngt=7,qg-,clc-,vs-,ql=1,hpfcvc=7,szrtb-,rx-,xff-,\
kqbrdm-,sjs=5,bqhj=5,nctp=7,lk-,cdv-,kbt=9,xk-,lsm=3,rpp=5,gqd=6,hr-,kqqj=6,hbztt=8,khg=5,fvqs-,\
hz-,rlk=8,dhq-,hgr=3,lsm=1,bpzv-,nt-,mdr=7,rj=9,slm-,blz-,pff=3,ztn=7,vvl-,qcmf-,fxn=4,\
gbql=9,qtr=4,vx-,zskz=7,hc-,nj-,pzs=4,pb=1,ntj=2,nkvb-,kg-,bpzv-,gx=8,bf-,zkd=9,cl=1,\
rdxc=1,cstn-,gm=7,xk-,vrf=6,grm=9,nndr=4,tlg=7,rxc=9,lqg-,xsb-,kvr-,kh=3,gmtcl=9,lqg-,jlq-,\
cl-,jcvsv=2,zxfss=3,fc-,jqs=7,rbt=1,cfnp=9,qtk=5,qg=6,hgjq-,txkg-,shh=1,vjl-,xp=7,tclhnp=2,kxl=4,\
rgb-,gd=3,xff=5,khg=1,fmc=5,rxn=9,fxllb-,jkb-,tgh=4,shc=1,zkd-,cstn-,xkhdmj=5,dnndg=4,qd-,qz=2,\
xkc-,fg=2,xbk-,ttrbn-,gsls-,shb-,vhdvm-,dbtjf-,rldhlq-,xff-,rd=9,zkd-,hc=8,qhrhkh=4,ggc=4,shh=7,\
jg-,hrf=3,qgcx=7,gnj=8,xgr=3,tz-,jgggh-,dg-,rkf=9,jh-,fpq=8,cbnpmr=6,pslcc-,qtk=6,gqd=3,fpq=4,\
cfnp=2,dhq=3,rf-,cz-,bvm-,rn-,kxl=3,txv=5,lpc-,qg=2,cfnp=1,slm-,ks=7,xlz=4,ppx=9,dls=5,\
zvb-,cbz=9,ntc=6,cstn=5,nl-,vxn=8,rkf-,rlk=2,tclhnp=7,mvmd-,rxc=1,tv=3,nr-,gnj=5,sq-,blz=2,\
ql-,ctf-,fc=2,hh=2,fdmgq-,tndl=3,df-,dbtjf=3,dls=5,jhfpn-,xqp-,pbk-,qgcx=3,lsd=1,kvr-,cbgpbl-,\
mdr=4,dnz=9,lkc-,jg=6,hnsrf-,cfd=8,vznn=6,glcb-,pdbqhz=3,phxfj-,nj=4,xsb=7,qv=3,lq-,jg=7,dhq-,\
tjs-,sl-,vd-,jph-,nxkb-,jqmtfm-,dcr=7,gtnhsv-,cvzr-,nmv=5,cpqz-,qjlpvn-,pd=6,pn=7,kg-,bbk-,\
kjmb-,blz-,ct=7,njfj=4,blsct-,fc-,xkhdmj-,bd=9,khcj=6,dlsvsf-,ntc-,dhq=1,fdmgq-,fnrbm-,lc-,jqs-,\
ngt=5,dhq-,qs-,cqn-,tr=5,ggx=8,bl=9,txkg-,dptc=5,pz-,zlj=4,hv=2,gd-,xzz=7,xkc=5,tp-,\
gm-,bmgs=1,rldhlq-,ntj=9,gpvt=4,mn=5,sq-,xcj=5,txv=3,fv-,tgh=9,pp=9,mgmns-,bxdg-,sdb=5,tzl=9,\
mjqn-,tzl-,dlx=1,sxm=4,dcr-,pcg-,tlg-,khp-,grxz-,cbbj-,cvzr-,zc-,gz=8,gmtcl-,gtt=1,sl=3";
}

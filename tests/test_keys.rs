use bc_envelope::prelude::*;
use indoc::indoc;

mod common;
use common::*;

const SEED: &str = "ur:seed/oyadhdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdweocfztrd";
const PRVKEYS: &str = "ur:crypto-prvkey-base/hdcxhsinuesrennenlhfaopycnrfrkdmfnsrvltowmtbmyfwdafxvwmthersktcpetdwfnbndeah";
const COMMENT: &str = "comment";

#[test]
fn test_generate_random_private_key_base() {
    let prvkeys = run_cli(&["generate", "prvkeys"]).unwrap();
    assert_eq!(UR::from_ur_string(prvkeys).unwrap().ur_type_str(), "crypto-prvkey-base");
}

#[test]
fn test_generate_private_key_base_from_seed() {
    let prvkeys = run_cli(&["generate", "prvkeys", "--seed", SEED]).unwrap();
    assert_eq!(prvkeys, PRVKEYS);
}

fn test_keys(
    key_type: &str,
    expected_pubkeys: &str,
    expected_signer: &str,
    expected_verifier: &str,
    expected_signature_summary: &str,
) {
    let pubkeys = run_cli(&[
        "generate", "pubkeys",
        "--type", key_type,
        PRVKEYS,
        "--comment", COMMENT
    ]).unwrap();
    assert_eq!(pubkeys, expected_pubkeys);

    let signer = run_cli(&[
        "generate", "signer",
        "--type", key_type,
        PRVKEYS,
        "--comment", COMMENT
    ]).unwrap();
    assert_eq!(signer, expected_signer);

    let verifier = run_cli(&[
        "generate", "verifier",
        &signer
    ]).unwrap();
    assert_eq!(verifier, expected_verifier);

    let verifier = run_cli(&[
        "generate", "verifier", expected_pubkeys]).unwrap();
    assert_eq!(verifier, expected_verifier);

    let signed = run_cli(&[
        "sign",
        "--signer", &signer,
        "--namespace", "test",
        ALICE_KNOWS_BOB_EXAMPLE,
    ]).unwrap();

    let expected_format = indoc!(r#"
    "Alice" [
        "knows": "Bob"
        'signed': {}
    ]
    "#);
    let expected_format = expected_format.replace("{}", expected_signature_summary);
    run_cli_expect(&["format", &signed], &expected_format).unwrap();

    run_cli(&["verify", &signed, "--verifier", &pubkeys]).unwrap();
}

#[test]
fn test_schnorr() {
    test_keys(
        "schnorr",
        "ur:crypto-pubkeys/lftanshfhdcxayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejobebtdlhd",
        "ur:signing-private-key/hdcxpfsndiahcxsfrhjoltglmebwwnnstovocffejytdbwihdkrtdykebkiebglbtteeimhsvamu",
        "ur:signing-public-key/hdcxayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdleryflfzdlia",
        "Signature",
    );
}

#[test]
fn test_ecdsa() {
    test_keys(
        "ecdsa",
        "ur:crypto-pubkeys/lftanshflfadhdclaoayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejomecapkpr",
        "ur:signing-private-key/lfadhdcxpfsndiahcxsfrhjoltglmebwwnnstovocffejytdbwihdkrtdykebkiebglbtteecsioludn",
        "ur:signing-public-key/lfadhdclaoayvazmflzsfrotemfxvoghtbynbsgywztlheisvapypmidzmaoldisdybkvdlerylgstinvl",
        "Signature(Ecdsa)",
    );
}

#[test]
fn test_ssh_ed25519() {
    test_keys(
        "ssh-ed25519",
        "ur:crypto-pubkeys/lftanshftanehskshdjkjkisdpihieeyececehescxfpfpfpfpfxeoglknhsfxehjzhtfygaehglghfeecfpfpfpfpgafwemeydlgseseyetendndlhkgdkpeojneseckogridjyjshsflgmjnidjzgdhgiygtgdhsgwhthkemdldyjyiycxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejogoimkkkt",
        "ur:signing-private-key/tanehnkkadmydpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpgtktfpfpfpfpjykniaeyiojyhthgbkgykkglghgoksgwgyfpfpfpfxfpiheskokkdliekogwkokoeyfyemjyeckoihidkkjnemhsjnisjehtjnecghehjtknfyeyimjnhggwdlesgshdktfpfpfpgefyjkeejpgyjlemgwgrdybkgrfpfpfpfpfpjykniaeyiojyhthggykkglghgoksgwgyfpfpfpfxfpiheskokkdliekogwkokoeyfyemjyeckoihidkkjnemhsjnisjehtjnecghehjtknfyeyimjnhggwdlesgshdktbkfpfpfpfefyidiaesjpgdjojpioioidjkkokpiaghgugsjsjlflkphdidiahtecihgyktjsinfxecehfgfykpdygheciegwfwemeydlgseseyetendndlhkgdkpeojneseckogridjyjsbkhsflgmjnidjzgdhgiygtgdhsgwhthkemdldyjyiyfpfpfpfpfweyglkoidhgehjzidjtgyfwfpiogtfefwgyhkfsbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkmnfptpeh",
        "ur:signing-public-key/tanehskshdjkjkisdpihieeyececehescxfpfpfpfpfxeoglknhsfxehjzhtfygaehglghfeecfpfpfpfpgafwemeydlgseseyetendndlhkgdkpeojneseckogridjyjshsflgmjnidjzgdhgiygtgdhsgwhthkemdldyjyiycxiajljnjnihjtjyeyflsegd",
        "Signature(SshEd25519)"
    );
}

#[test]
fn test_ssh_ecdsa_nistp256() {
    test_keys(
        "ssh-ecdsa-p256",
        "ur:crypto-pubkeys/lftanshftanehskspdihiaiejkhsdpjkishseydpjtinjkjyjoeyecencxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpkkglghhkfpfpfpfpgaidjnjzkniefdfpkkglghhkfpfpfpfwfwfwflengakpflgygoiajteyeyehgdhkimgwgeihgmgmingeiyiyhtjzjnksfxjkemfwenkphsgsksjtgyeyioeteokohsidjykpehgdihgsiyguenjsjtgojofgjlfggadyjedlgrioetgahdemjyihjpioidjtidkpgajkglgafektfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejodlztswca",
        "ur:signing-private-key/tanehnkkadwpdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfphsfpfpfpfpfwgljzhkeygmknhkgubkehknhsflfekkgshgecjoiaeogmktgtimgoeyfpfpfpfpfxflecjoiaeogmktgtimgoeyfpfpfpfpgygygmkpingsisjefgfdgeesjyjygheygakninhdjegohkinhdeoeyhthtjkgyjpbkgwktihjpjninethtdygljlgdglemeyjnemidjygheoineodykpjsjoehgrgmhsfwguglgegdkkjlgdfxfgdnemhdjseefleceyemingsfygufwgtfpfpfpfpjlgwhggsfxgykojzinktbkjegsfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpkkglghhkfpfpfpfpgaidjnjzkniefdfpkkglghhkfpfpfpfwfwfwflengakpflgygoiajteyeyehgdhkimbkgwgeihgmgmingeiyiyhtjzjnksfxjkemfwenkphsgsksjtgyeyioeteokohsidjykpehgdihgsiyguenjsjtgojofgjlfggadyjedlgrioetgahdemjyihjpioidjtidkpgajkglgafebkktfpfpfpfpiofdjeeegreeengtktgleejpeoiaesimjnjtjteyjzglkkghimhkiydnemdlgaghgdindngsiojlkkgoflghfxfpfpfpfpfpfdhkeyesjyidhghfkpiefpfefsbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbktynbssur",
        "ur:signing-public-key/tanehskspdihiaiejkhsdpjkishseydpjtinjkjyjoeyecencxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpkkglghhkfpfpfpfpgaidjnjzkniefdfpkkglghhkfpfpfpfwfwfwflengakpflgygoiajteyeyehgdhkimgwgeihgmgmingeiyiyhtjzjnksfxjkemfwenkphsgsksjtgyeyioeteokohsidjykpehgdihgsiyguenjsjtgojofgjlfggadyjedlgrioetgahdemjyihjpioidjtidkpgajkglgafektfscxiajljnjnihjtjyrhpttnon",
        "Signature(SshEcdsaP256)"
    );
}

#[test]
fn test_ssh_ecdsa_nistp384() {
    test_keys(
        "ssh-ecdsa-p384",
        "ur:crypto-pubkeys/lftanshftanehskstyihiaiejkhsdpjkishseydpjtinjkjyjoeoeteecxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpkngwfygyfpfpfpfpgaidjnjzkniefdfpkngwfygyfpfpfpfwisfwfeeedyfgdyksjogrgmgoghhkieidindlgrisfggdjofwihehgwidjohsgsgleojlgwjlflhdhfjehkkoksecgmididkkengdjkiaflgehtemflgtdyingsguihinjojljletkkhdgyemfefwgogmksinflgaingokojliyhgkojyetjzeefggmehfleyksgaiogadyimiefxiseteyfggejkiegoehkkfgimethtiajkgtindlisjnecghdnjthsiofsfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejoltvdgula",
        "ur:signing-private-key/tanehnkkaohtdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpinfpfpfpfpfwgljzhkeygmknhkgubkehknhsflfekkgshgecjoiaeogmktgtkniodyfpfpfpfpfxflecjoiaeogmktgtkniodyfpfpfpfphkgygmgwglfwiegthsgujehffeeyfdhgeekokkjlgmghengyhdjyghjnenhginknbkieenfyjsfwjzehhtflgsetihgohgeyetkpimemfdfwinhgihksimglgaindyjtjljshsgrgdgtjzdygwksfphffeiahkisingajzgsenfdehjpemiygeihfwgoiegmjyjkgugafxglgabkeogyjliyglisguidfdhfgliaishkdlflhdgsfygakoeehtkpgodljoeyjlfpfpfpfygyioeseefegtgagdihfwfyfpfpfpfpfpghhthggljeiaeyfejyiaeyisisgtinehkphshdgldybkiafygteeglfpfpfpfpfpiskphshdgldyiafygteeglfpfpfpfpflfefeghimgyhdghfljejofggmglisehkpgsetjsfegodnjefgemgoeckpjzjljkeoihioeniohtiehggmindlfdjzbkfgjykogsjldnkskthkjzjtjkhkkngugajygeengrjninimkngeiefyjkgyfggmfdflgahkingegudniseshsdneokkhdiohffdgoididfeinfpimgugldygrfdknhkgojnksehghhdgabkhggdksjzkkktkkgsdnflidjzgdeniejsfpfpfpfpgtgyfxjljejtjygdinidgmfejliaiadlehglioiejofddnhdkojleejpjejehdenglesjyjofldliagrjljkgeisdyjeksjekojybkdyfgihkkesktknhkioehfpetkshtkojseefpfpfpfpfdhkeyesjyidhghfkpiefpfsfsbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkkomhbwty",
        "ur:signing-public-key/tanehskstyihiaiejkhsdpjkishseydpjtinjkjyjoeoeteecxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpkngwfygyfpfpfpfpgaidjnjzkniefdfpkngwfygyfpfpfpfwisfwfeeedyfgdyksjogrgmgoghhkieidindlgrisfggdjofwihehgwidjohsgsgleojlgwjlflhdhfjehkkoksecgmididkkengdjkiaflgehtemflgtdyingsguihinjojljletkkhdgyemfefwgogmksinflgaingokojliyhgkojyetjzeefggmehfleyksgaiogadyimiefxiseteyfggejkiegoehkkfgimethtiajkgtindlisjnecghdnjthsiofsfscxiajljnjnihjtjyaagtkkvo",
        "Signature(SshEcdsaP384)"
    );
}

// Disabled due to a bug in the ssh-key crate.
// See: https://github.com/RustCrypto/SSH/issues/232
// #[test]
// #[ignore]
// fn test_ssh_ecdsa_nistp521() {
//     test_keys(
//         "ssh-ecdsa-p521",
//         "ur:crypto-pubkeys/lftanshftanehskkadaaihiaiejkhsdpjkishseydpjtinjkjyjoeceyehcxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpehgtimfefpfpfpfpgaidjnjzkniefdfpehgtimfefpfpfpfxfgfwfpfegsgmgrktisgdjpflfwfxdnjkemimhgfeisknktgweodlethkgtjeiagwiagygafgidiefphtehgaisguksjogeiehkfygtguhtjpgdidjkgdieenjsjlkpghimksgdgsiaemjehgknjnimjsglkteyiykoglfpjodydyetkpidjygyfpfgkoiygljkjsgafeiejteydninjzfdeohdjtfgglenhkksgajlihgudyidgajsghecfxfgdniykkgeehgrieiegmkpjyiskkehjtgrkkgyknimgddnjpfyeyeeihfgemghgmgogwhfghkpinhgjpeoiykpdyiydyktimgaehjygygyfsfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejofrfpkery",
//         "ur:signing-private-key/tanehnkkaotydpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfpjpfpfpfpfpfwgljzhkeygmknhkgubkehknhsflfekkgshgecjoiaeogmktglghgaksfpfpfpfpfxflecjoiaeogmktglghgaksfpfpfpfpisgygyfwfxdygujkgaghenksiogykojpgweeehisgaiaetfyjydldlflfygefdfybkjtfefxfwhgeogyfliegugagojkhsguhdhgfpknfejnhskneyemfyeoihjsjsgsjeeeetghkkeogwecfgjkecjlenimiagljtemkngygrieglgdgsjnemgofpfwideoknidgrinfwfdhtbkeskojljogmesehecksghihjngtgugrfdjejyflkkgrjedngyisiyjtetiniegujthdgoidjphkiajyhtkkjkjegteekndljskteskpfdisihdydyhffyjzgoemjljzjseseoemjyfdesbkgtgakkglidgofefpfpfpfegaknkkeneekkjketkpkpgtjlfpfpfpfpghhthggljeiaeyfejyiaeyisisgtinehkphshdgldyiafygokkgtgyfpfpfpfpiskphshdgldyiafygokkgtgybkfpfpfpgagofefpgyjyfejpfxfednjkhkfegsenknkpglhkgufdgdfpemiydlksiokkgmktecksfpiohfjydyfwjtgoinfggsfljejzehiogtksgejnjkeskpkteseojsjsinecgwgdfebketjyknkpgmidgwhsgwjleofyhtdnetdyfxjtghghkkeckpehfpfphgeseteykkjliogmeyiyidengrgoiyieihiagoeojoimfeinisecgsgmjkinjogdjegahdecdlgajtgojoehehflbkeneyfdgshgiajpgefygwgtdlenjkgdidiseehdjyglfggyechfgwengehskoiednemgmdlghfxgtimhgehfwfpfpfpfpgyiofdjpjpjkjyeydydlhgideceskkemjpieehihflhsjyeybkjykphggeknisjliyjydlhskkkteyjeihjzfwgtkoiaisesfefgisktjofeesihfdkkksghiagyeegefwksgtieflfphkeedlioihgydlhgisengwjoiejseefpinjzgadnfpfpfpfpfpbkieimideyehjyhthgecdyfpgygafybkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkrevawsrl",
//         "ur:signing-public-key/tanehskkadaaihiaiejkhsdpjkishseydpjtinjkjyjoeceyehcxfpfpfpfpfeeyhfimhtfdglisgshdgljlhkghgajyidjnjzkniefdfpehgtimfefpfpfpfpgaidjnjzkniefdfpehgtimfefpfpfpfxfgfwfpfegsgmgrktisgdjpflfwfxdnjkemimhgfeisknktgweodlethkgtjeiagwiagygafgidiefphtehgaisguksjogeiehkfygtguhtjpgdidjkgdieenjsjlkpghimksgdgsiaemjehgknjnimjsglkteyiykoglfpjodydyetkpidjygyfpfgkoiygljkjsgafeiejteydninjzfdeohdjtfgglenhkksgajlihgudyidgajsghecfxfgdniykkgeehgrieiegmkpjyiskkehjtgrkkgyknimgddnjpfyeyeeihfgemghgmgogwhfghkpinhgjpeoiykpdyiydyktimgaehjygygyfsfscxiajljnjnihjtjyfhayjkjn",
//     );
// }

// Particularly slow, so disabled by default.
#[test]
#[ignore]
fn test_ssh_dsa() {
    test_keys(
        "ssh-dsa",
        "ur:crypto-pubkeys/lftanshftanehskkaoghjkjkisdpiejkjkcxfpfpfpfpfweoglknhsfxehjeiaeogtfpfpfpfxfwfpgdfwgretjtjphtjehdiyieksjykteygaingmhdhkiaimidemfyfghsfwjkiejzknhkjohgksghgmeyesjekkgygyfljyeyhkeneeinkseydndnfxjnfxjleeesgeesgygujofdkthgiohtgofgechffwhfgoghfyghflflksgdemksksfeguecihgsgudlhkiyiykpgmemethkhdiohtfeidfpguecjsidglidhdhkfdhkhtfehfhgiygwfdiyghjsgmfyfxgmgmjpjegageesgegoinjtflghfwglgedlgaknfydnjpihehendlfyieglgskkjnhfhtkteokpfxjyfpfpfpfpfggyfygofletgukkfeihdnfphgjtisgtjoiodlkkjykoglkpecgsktknetktfpfpfpgafwgmkogmiaehglhghthkjlgrfxfgetiegdhkgogrfghdhgfwhsjsinisgshfhsfdhsktiyisktfdjehdfpetjsisgaeskojsemgujoemhsdlkphsjnjziegmgskojlkofdfwkoecjlecgrgyfdjpguhgemiojlglfwidkkgafeenimjzfyisgweoiehfhgktfxinidjygufyfwiaksgegljyhggtgaetkoeejsiahkfgdnkpeekkjefgingmiajnfwjektgyjoihemgukngljlglfdhffxeyflfwgreoglfgjnimkniegeisfwhdgmenfyfdhkkseeecfpghieksihiofpfpfpgafefpjlidglfldlehidfpeekogseohdechtfdfyhthsgwfwetgmemjphdjohgihihgednishgfwkthsimfpfddlhgimiheoinjzehgljzjlgyihfeihgsiojpgwimjohfjnfgdldlidiagmisieiykkkofdghiygyjlgreyiokteseefgehgwiakphtghemktflkkgyeojzgdgahsjzgeehjejokngretjsdnimfygddngliagrhsgsioghinehimjsiagridglgukpfwfwjlineteyehfphdghimidhgeohkgdfghfhsenhfhdgwjtgtgmjnfwinjyjzjzfygrjoeyfyetfscxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejomsaadihh",
        "ur:signing-private-key/tanehnkkahhldpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfwjkiofpfpfpfpiekniaeyiojyhtfdbkglknfpfpfpfpiogyfyktgukogeeneyhtfgeoeoiaidiaglingajehfeyfdgaeydnktkshgioidfdhtiaeygrhfjkgodyiekohtgtjefefwjpiejngwkpgajkiekokoiojoiojsgwgdgubkiygofejsgmetfgjlflhffwihhfgyhfhffektdyksisjkghdnetiagmfekphdindykoeyfdeoemjeihdlflfgeeflgmflktfekphsjnknhgeheyfweyflgmfghfjtkniseodyenjegyktbkjegohsecfxfxiyguhfgajoksjektghguiykkgtktdljseojyihkokteoghguetjojzhgiaglemiojpgyfpfpfpfwgofpehfwkofejkisfdkoiofgjoeeghgrhkgdetjpidknidkpguetbkgtdlgtfpfpfpfxfpgoiddyhdglghhfjnhggrfxioisiyfdgheyfgfxishfehiohgjsjljlguehhgiseyjkfdeeiafwecfgktgdgrjlgugdidenkpdyjsiheykoemjnjojohdgoguemenbkgsksktiddnhsgwgujefwendyjzkpeegrfygyhgetinfwgwjlecgyeeghjyeohfhfjkfpjljnemgoiokthdgtgughidhfimfxgdgsdngrjtflfwiyjpkpgtjofwhkjehdgeiohtgtfegrbkhdkpdyjkknhsfygmehgyjyisiogujykngmhtjleteoguhkgyhfdyihiokseygtihgwgyfeeoiahdjlfpfpfpfxfwfpgrflkngmkoeshgktgwgskkesehdnhggmkteyhgimioiyfeihenbkehenhfjtjtiniyjlhfioiafljlktfwdlehjleojyeejoieghhthsfefdisfdineegrknjlenhfhtisiydleyeofehkhdhdetjpksdyeodygrfxjyjlgtgdihfwieghjtgsjngodnetfwbkjkjeglecghkkfljoguiehtgriakkkogrkojlktkndlimhdfxjnineefeeejyhkenjtfxjnkngojpiogyhsgakogljygyfgdyeeeyehjyeyfykshfhgkpjzhfknjoknfehtiohkjphthtbkgykkjsieiodlfpfpfpfweegegtgegtfwgrghfxghfpgufpfpfpfpfweoglknhsfxehjeiaeogtfpfpfpfxfwfpgdfwgretjtjphtjehdiyieksjykteygaingmhdhkiaimidemfyfghsbkfwjkiejzknhkjohgksghgmeyesjekkgygyfljyeyhkeneeinkseydndnfxjnfxjleeesgeesgygujofdkthgiohtgofgechffwhfgoghfyghflflksgdemksksfeguecihgsgudlhkiybkiykpgmemethkhdiohtfeidfpguecjsidglidhdhkfdhkhtfehfhgiygwfdiyghjsgmfyfxgmgmjpjegageesgegoinjtflghfwglgedlgaknfydnjpihehendlfyieglgskkjnhfhtktbkeokpfxjyfpfpfpfpfggyfygofletgukkfeihdnfphgjtisgtjoiodlkkjykoglkpecgsktknetktfpfpfpgafwgmkogmiaehglhghthkjlgrfxfgetiegdhkgogrfghdhgfwhsjsinisbkgshfhsfdhsktiyisktfdjehdfpetjsisgaeskojsemgujoemhsdlkphsjnjziegmgskojlkofdfwkoecjlecgrgyfdjpguhgemiojlglfwidkkgafeenimjzfyisgweoiehfhgktfxinbkidjygufyfwiaksgegljyhggtgaetkoeejsiahkfgdnkpeekkjefgingmiajnfwjektgyjoihemgukngljlglfdhffxeyflfwgreoglfgjnimkniegeisfwhdgmenfyfdhkkseeecfpghbkieksihiofpfpfpgafefpjlidglfldlehidfpeekogseohdechtfdfyhthsgwfwetgmemjphdjohgihihgednishgfwkthsimfpfddlhgimiheoinjzehgljzjlgyihfeihgsiojpgwimbkjohfjnfgdldlidiagmisieiykkkofdghiygyjlgreyiokteseefgehgwiakphtghemktflkkgyeojzgdgahsjzgeehjejokngretjsdnimfygddngliagrhsgsioghinehimjsiagridbkglgukpfwfwjlineteyehfphdghimidhgeohkgdfghfhsenhfhdgwjtgtgmjnfwinjyjzjzfygrjoeyfyetfpfpfpfpgogygtgoflioesfxiefxiyeoeojzgejsfgetgsingdjyfpgmesbkgrjzdyfpfpfpfpfdhkeyesjyidhghfkpiefpfefxfpktfsfsbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkkpsrguhk",
        "ur:signing-public-key/tanehskkaoghjkjkisdpiejkjkcxfpfpfpfpfweoglknhsfxehjeiaeogtfpfpfpfxfwfpgdfwgretjtjphtjehdiyieksjykteygaingmhdhkiaimidemfyfghsfwjkiejzknhkjohgksghgmeyesjekkgygyfljyeyhkeneeinkseydndnfxjnfxjleeesgeesgygujofdkthgiohtgofgechffwhfgoghfyghflflksgdemksksfeguecihgsgudlhkiyiykpgmemethkhdiohtfeidfpguecjsidglidhdhkfdhkhtfehfhgiygwfdiyghjsgmfyfxgmgmjpjegageesgegoinjtflghfwglgedlgaknfydnjpihehendlfyieglgskkjnhfhtkteokpfxjyfpfpfpfpfggyfygofletgukkfeihdnfphgjtisgtjoiodlkkjykoglkpecgsktknetktfpfpfpgafwgmkogmiaehglhghthkjlgrfxfgetiegdhkgogrfghdhgfwhsjsinisgshfhsfdhsktiyisktfdjehdfpetjsisgaeskojsemgujoemhsdlkphsjnjziegmgskojlkofdfwkoecjlecgrgyfdjpguhgemiojlglfwidkkgafeenimjzfyisgweoiehfhgktfxinidjygufyfwiaksgegljyhggtgaetkoeejsiahkfgdnkpeekkjefgingmiajnfwjektgyjoihemgukngljlglfdhffxeyflfwgreoglfgjnimkniegeisfwhdgmenfyfdhkkseeecfpghieksihiofpfpfpgafefpjlidglfldlehidfpeekogseohdechtfdfyhthsgwfwetgmemjphdjohgihihgednishgfwkthsimfpfddlhgimiheoinjzehgljzjlgyihfeihgsiojpgwimjohfjnfgdldlidiagmisieiykkkofdghiygyjlgreyiokteseefgehgwiakphtghemktflkkgyeojzgdgahsjzgeehjejokngretjsdnimfygddngliagrhsgsioghinehimjsiagridglgukpfwfwjlineteyehfphdghimidhgeohkgdfghfhsenhfhdgwjtgtgmjnfwinjyjzjzfygrjoeyfyetfscxiajljnjnihjtjycfeensbg",
        "Signature",
    );
}

// Particularly slow, so disabled by default.
#[test]
#[ignore]
fn test_ssh_rsa_sha256() {
    test_keys(
        "ssh-rsa-sha256",
        "ur:crypto-pubkeys/lftanshftanehskkadlrjkjkisdpjpjkhscxfpfpfpfpfweoglknhsfxehkkiaeyfefpfpfpfpfyfpgyfpfwfpfpfpfwfpgyfyjzktinkkdyhfishgdyhgfwfxhseejkfwidksjsfyidgojyenjsiheeenjsfgjogujnececjykkglgsjsideseyhkhsjkesfgiyjejnfekpgsiakofpgseheoeyeehdiahfdygefdgeioehfpidfxiaenjtiegejkeegahshtgrjyiehgjshgksgsdnemehineydyjkktinhtdlfgjngakoktgafefyfdgwgwgwhtjpidendygueyimhgfddngmhkgoesjlgedydnkseohkfygskkhkinhgjyesfxkogwiajogtisihkphdetehgtgtdyflhkjpfdjzjyiogoisisesktjtengrfgjyjeknjshsjnjlgaeninglfgjsgthfdyecglgegegreeimfygygahdjkiejldyjlfyetfygrimgagdihkohgjtguiofdisiofyjtfgidgmeednfxkphkendygtjoidesgogseoineoktflfykniyjnflingejtjeeokkioknidgoetjzesehengojeetgoihkoidfyhtecgojojokkeshdhdjnehiofefwdnfygyiejzkshthtgeknjeiyisfygufpdlksenghgdjojygteyioesgsetfdgugmeogejneeetjkgtecdnenjkjoehhdgsfwjeideyfdcxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejotpgdosen",
        "ur:signing-private-key/tanehnkkatbwdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfwfgktfpfpfpfpiekniaeyiojyiajtbkglisfpfpfpfpfpktfefpfpgyfpfpfpgyfefpeciagajkjyfghkhfjyfgiogyjnkpgsfphgethsioeyehgsihjsjtkpgwjsishsgojokpihidiaimguenjndliejnfljpgdgmhdecgeisbkgsineogsktfxesieeskpfgeofgiefxgmkkhkglgyflktjtgwjoeoguidgwfxfljngujphdhfjsjzjkgudlkpeshkjyjygsgtgajniykshtingsetfxfwfpksknimimjnhseydnjyfejybkjlehisdljehgfggdhsfxiegdjkieeyfpkketjngajzjpiygyjpknjtgrghgahdjpjzdlglghfyglfwjngrksecidhkfggahkiyiagedninisidhtgtenjnjojsfxgwjlimgmhsimfgiebkgwghgugugukpgaktdyfxfgemfdhsglgrfpdlfpkkjlkkfyeojpehjodyjlfweehkfpeckshgdyihgdiojpjngwjyfygrhgdlhffxeseejyetfwioeteoecisjlinhtecgletjlgteyehbkgdgeiyieihjzgegdfgfdjpeykteyihhfgrhsiakohfehecjyhkfwfpiyiodyfdhtiahghgguiaecfdeegydyiogdetihjeknenidghgljlgdgudlfwdyjeiekkhtkpgdgsfygwiykpjpbkgriehfkkkthtflesisktfpfpfpetfyhkghfykpdleyfektemkoktfpfpfpfpiekniaeyiojyiajtglisfpfpfpfwfpgyfyjzktinkkdyhfishgdyhgfwfxhseejkfwidksjsfyidgojybkenjsiheeenjsfgjogujnececjykkglgsjsideseyhkhsjkesfgiyjejnfekpgsiakofpgseheoeyeehdiahfdygefdgeioehfpidfxiaenjtiegejkeegahshtgrjyiehgjshgksgsdnbkemehineydyjkktinhtdlfgjngakoktgafefyfdgwgwgwhtjpidendygueyimhgfddngmhkgoesjlgedydnkseohkfygskkhkinhgjyesfxkogwiajogtisihkphdetehgtgtdyflhkjpbkfdjzjyiogoisisesktjtengrfgjyjeknjshsjnjlgaeninglfgjsgthfdyecglgegegreeimfygygahdjkiejldyjlfyetfygrimgagdihkohgjtguiofdisiofyjtfgidgmeednfxkpbkhkendygtjoidesgogseoineoktflfykniyjnflingejtjeeokkioknidgoetjzesehengojeetgoihkoidfyhtecgojojokkeshdhdjnehiofefwdnfygyiejzkshthtgeknjeiyisfybkgufpdlksenghgdjojygteyioesgsetfdgugmeogejneeetjkgtecdnenjkjoehhdgsfwjeideyfdfpfpfpfpfpktfefpfpgyfpfpfpgyfefpkpiygogleoenkpjofweodyidiheofxecbkgmiaenihehgtghihkkhfiahfjlieenfxehfpdnkkkshteyieksenkshggsgeingsisjtfdioguesgskofygyengehgetetinhtimjshkglflgyihinjpgdflhgiaksgsetgyglkngeksbkidfxeceeeyjketknesiygrkkflknemgtimenfdidgwfgfyknhgjeguioiygljtkojehfgwgrgdenfdhsgeisehdlihjljlgwkpengdgwjlhsfdjkesinihkkglgefwkpisjpetfwidjkbkkphgehhffeenkndlehguetiagwioghflgrjsgoimhgfejsfpgyenjkehioememhdjpjohfjskshfjsfljniydyfxflktgmhdfeenfpeciofweygtfejlehgyjtinjyglgwemimktknecbkkkinimetdngugmjejnfwihgtjlhsgmjeidgednenjykneoeejtkohkjeiehsfejtenfggsidgagrjlgrjkfgglgaesjsgskohsjofpecgthsechsemeyisjtjsehjkgsenfdjtgahkkkbkfxehetdnghfggwkpeofehgjtjpehehgeeegoguengsgejohsjeksgtinfdgtisimdlglguhkdygtkpjlisjydyisfpgyfpfpfpgafefpjefpdyinfegajektjofdiehsimjpghknemgrbkgeeydyhdktjnfdkpeyguhgjtesgtfxgletkkihdlihimhdioguetjtfgjyeshdiajtjtecjoiseeiajsimkpgsknenjygskkiajoeyjefwgegsiefwfwkpfygukkfwfxgefpihetghhfbkgljygegtjyideekkdngsfljzghecjtkodyfyguguiefwgyfydnhsiyghetflknfdfgeoiadyiygajyethsfdfxfxgajsididfyimecidihghidgtjoeogwfyjlglgaghgtehiefdihhtbkgldlgrkofeisgljnidhkhtdlktfpfpfpfxfwfpgdksimeyidkpidjogliafphkihghhkfpjeemkneskngajsesiejkjkgrgegtetgahtflfwfyimflhgfejokngykthsgshffwdngrhfbkeneyfxineeksjliafgesesesimeshshsjziejzdyjofpeofwgsiajyfdgohdjlendngujkjtjyjsfpiheoehhdjojsgtfpgdiojeghhffdgtisjneoemecgagseokkfxgygsfleciyeebkisihgojthgesjedlecjygsgdhggeglgljnidingmjyhghfjohkfdgljphtgaiohdgdjkjyfyghhgehjsjzgejnetgagmfdjngufwfpfpfpfpiogyfyjofxeoghjegteodleeimioktgabkjnkthtihkpgrfljzisknkpdlgoemiojkfximimgleefpeniahgfwgtioioeeeneofxgogljzfygeingwkkgogujzechkisjygyflehgogsflfdjnjthfimiekojlgujnfxiejskkkkghbkhgingehtjeimhkjsjodyjykpgddlfdguguieinhsemjyiaeygueekpjofgjtkkhdgudndnkpkoeefwhdgdingsgsjsgtemkoesidknehjejogdfgjojsdygdkkfxihidglehhshtemdlbkdyknjpecenguiaisetiaesiahseehddnfwktfpfpfpfpieimideyehjyhthgecdyfpgygafsbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkjzdmdpmh",
        "ur:signing-public-key/tanehskkadlrjkjkisdpjpjkhscxfpfpfpfpfweoglknhsfxehkkiaeyfefpfpfpfpfyfpgyfpfwfpfpfpfwfpgyfyjzktinkkdyhfishgdyhgfwfxhseejkfwidksjsfyidgojyenjsiheeenjsfgjogujnececjykkglgsjsideseyhkhsjkesfgiyjejnfekpgsiakofpgseheoeyeehdiahfdygefdgeioehfpidfxiaenjtiegejkeegahshtgrjyiehgjshgksgsdnemehineydyjkktinhtdlfgjngakoktgafefyfdgwgwgwhtjpidendygueyimhgfddngmhkgoesjlgedydnkseohkfygskkhkinhgjyesfxkogwiajogtisihkphdetehgtgtdyflhkjpfdjzjyiogoisisesktjtengrfgjyjeknjshsjnjlgaeninglfgjsgthfdyecglgegegreeimfygygahdjkiejldyjlfyetfygrimgagdihkohgjtguiofdisiofyjtfgidgmeednfxkphkendygtjoidesgogseoineoktflfykniyjnflingejtjeeokkioknidgoetjzesehengojeetgoihkoidfyhtecgojojokkeshdhdjnehiofefwdnfygyiejzkshthtgeknjeiyisfygufpdlksenghgdjojygteyioesgsetfdgugmeogejneeetjkgtecdnenjkjoehhdgsfwjeideyfdcxiajljnjnihjtjywdurdkdy",
        "Signature",
    );
}

// Particularly slow, so disabled by default.
#[test]
#[ignore]
fn test_ssh_rsa_sha512() {
    test_keys(
        "ssh-rsa-sha512",
        "ur:crypto-pubkeys/lftanshftanehskkadlrjkjkisdpjpjkhscxfpfpfpfpfweoglknhsfxehkkiaeyfefpfpfpfpfyfpgyfpfwfpfpfpfwfpgyfyfdgukteygrfliegtfwiyhshfglgoimhfiodliejogsjlgaeyfyeogljnfdingrdlgsingwfwhdkogwjthkfydnksgsgdgakokkhtgwghkpeskshtfldlgsktknfpdyjkgrghghjsgsjnfyemimksgwhkfegufgfgenhkemeyetfwetjnjsenfgglflkkgekkgshgimfeemkneoieiyhkgwkkgtehgrghktgrfxfwiminhfemhtfwgmiyflfphfgljodydyglgogmeciaglhteofykndlgdknjnfwihdyjejpflgtgdeokpdykkgaiyjnfygrhtgufedliyeygrecgdkofygrdyjsfyknguhffdflgugrdyhgkpgokniegdjzidfdhsehdneyecidksfgdlhshsehhsidiyfyemgwiyhsiefegrgdhgihguiekokkhkfeeoeyfeknfygodlimgofdghjnemiajliakkemgmeeghgtesdnfefeiyeekkjzfgidinhkjphsjtjtfpgwfljykpfefgesgyfdenkojokkdyhtjkgagtesksiyjpjeioeejefpioiodnidjkimetgsdlgdeefwjsetguecgdececdlkojsgtisioiehkhtfxiajyjegoksgyeygljkgwkpihhsfyjkiohtihgahfcxiajljnjnihjtjytansgrhdcxfdgwgacloxsrmupdcybdchfylewsdilrbestjodpwnknndjoztjprfkkjopkdejovyotplpd",
        "ur:signing-private-key/tanehnkkatbwdpdpdpdpdpfwfeflgaglcxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkideofwjzidjtglknhsfxehjphthdjejyieimfefpfpfpfpfpfwfleckoidjngofpfpfpfpfeidjneskphtgyfpfpfpfpfpfpfpfpfpfwfpfpfpfwfgktfpfpfpfpiekniaeyiojyiajtbkglisfpfpfpfpfpktfefpfpgyfpfpfpgyfefpksdyjkglinisjtghfphdeyjzghhfgaehhkgdeohsguenfxglioesknhtiseeinkokkeeimiohfemknjoeyfpdljkguknkkgsetjnghjebkemkoiahggmkokketgtktglgsfxjedyenineciodneeetghjnfwfeisgmihjngweskofpiygejskpisghgmjkiniainehjlksgweteseohdeyfyjkimglgujeetfxioiohkeejziheygybkgohdksiofgghhsieglfyhffeihhdfyhgiektetdlkneteciohdjygegrksimfyesemjygtinfdeciokkjngoisgdeoesinkpghemktkkjygrioetdyjzgmksjeinjyfgjpjzgteoghecbkhgkseyjyiyjykphgetgmiyeyjnjyhgjneoktdnknjteyjtgmfximehjtjejtidetjnfwglesisgtktehgdeeehfwdyeckpeogrfdgtkpdyihfekngdiyisfwfddngtjogmhgeejngreybkjoecktfyisjpidisfwiygofwdnjpeniajyflidfxfygdiahdenecgagwgefpgagagdjnemgadlfxdlkndnfphskofekpghdnihiyemenimgahkfdhgflgyjtgshtfggtgoglimidfyjpbkjtjnioemgaflhdinfggyfpfpfpetfyfpimioeygmktgaeegljegyfpfpfpfpiekniaeyiojyiajtglisfpfpfpfwfpgyfyfdgukteygrfliegtfwiyhshfglgoimhfiodliejogsjlgabkeyfyeogljnfdingrdlgsingwfwhdkogwjthkfydnksgsgdgakokkhtgwghkpeskshtfldlgsktknfpdyjkgrghghjsgsjnfyemimksgwhkfegufgfgenhkemeyetfwetjnjsenfgglflbkkkgekkgshgimfeemkneoieiyhkgwkkgtehgrghktgrfxfwiminhfemhtfwgmiyflfphfgljodydyglgogmeciaglhteofykndlgdknjnfwihdyjejpflgtgdeokpdykkgaiyjnfygrhtbkgufedliyeygrecgdkofygrdyjsfyknguhffdflgugrdyhgkpgokniegdjzidfdhsehdneyecidksfgdlhshsehhsidiyfyemgwiyhsiefegrgdhgihguiekokkhkfeeoeyfeknfygodlbkimgofdghjnemiajliakkemgmeeghgtesdnfefeiyeekkjzfgidinhkjphsjtjtfpgwfljykpfefgesgyfdenkojokkdyhtjkgagtesksiyjpjeioeejefpioiodnidjkimetgsdlgdeebkfwjsetguecgdececdlkojsgtisioiehkhtfxiajyjegoksgyeygljkgwkpihhsfyjkiohtihgahffpfpfpfpfpktfefpfpgyfpfpfpgyfpgwiehsinhgdyeeidhgkoiejlfdioguhtdybkgoisidgsjednkpgyjtgrecdlktiyfphdgohgiogljzhtkshfeyesjtjnfdeeesemehetktktiejnidethkhkkoenfleektgdgmfejekkgehsjskkieihdlfliaiaghgdjteojpesjkgabkideygujlgseegugtiyjsenfdfghkfxgehtesjsisihiyjofygagdhtjsjtjsemkphgiedyehkofgglkthfjogoghhgenhkgsglhtdnjoeciahthkihjteoglioeyecdliagtfxgseyjnbkdlgahkgydngukshkieihfeeyihhkgdkoeeetkshsgugudlesfyjzflgmgdiadljykpiygefxjeguhshghfhtgleydlisgsglfeiefgjkjpjpjljkgljehfisfxfeiyksgyemgteckkgsbkghjtgyhkhtfpflgsjzethgksgujsglihguhgkkksflfwhgjedliajnkpfyfdeoehimeodlfljzgygwiainkogsfweciyfpguecdleniogwiegdetfeksgwgwgahggwiyjtktisiyjtjnbkfddlhdjeglfgetknghghfdesdyjohgeojyisiefghdjskniniykokogogafwjpfegoflhdiaishfjzdyjydnjefwfpfpfpfpiofdgskkhgktgoidiadnjljsjlhgjyghhseyinjpehglbkhkfykohtetgaemghgwjoemhggrjnkkfdgaksidihjngakpfleyktjpidjeesjnidfefgdnhdgmdygefggegdkngsgegdiafydyfefgemjsiejtesjpgdgojejeeeiydldnihimglinidbkguimengaidenisjogehtdlghingwjnjsjsfdgyguisjzesjtknglksgrhgidihjehgjpemioiykoflhtgofdkoioksjnecjnfygsetdnishdhskkglgdgwfwgmgrgdiyfeksinjegsdybkjtjyflkndnktgtgseyhsfpfpfpfpiogyfyishtgsecjeemjoieiejljtjtjliskkiyjsesjogdemfwgljnfweefpdlgoglfwetgeesdlhddyhkenfxehhgioimjkeedyecieiyimkofxbkisfgjsfygeghiagaglhdeogseygofgfyknecjyechkgsjtjyjsglhsjngwfwesdnfgfwesguinhdfejtfxjlhfjyfxhtecgmktgugldyeekokogegegajyjpktjyghhdimjkjejoecktbkjzimknidgtgahtgwghinktioieihfxdnemjlhfeefxeyiejlfxdyfgehfefgfxiygrflfygyemdlkkfpiaglgakkjziehfgyfpfpfpgafefpeejzjpdnieimeydlihecksgyiheefgesbkgliohdiaesglhfeyjshkhfemjyjyknenfdjpgrjpgweojeiagyfdendnesdnghkkgokkjzjljkhtgegtjpfygdhdeyjsgaehgmksgujnghjsjpjshtjpesjnfwdnjogyfdfwfeemdygabkgygtehflktieinimkkfwgegdetinfwghgheseskphteneehtenihjpiyjpjeemgtglidgdjsetgyktflgafwiyesjyguisgsgegyghimfehkeeisksgefejykpksgsetiniejzhsglhtbkhtjegtgahseygehkeeetkodyiaiafefpfpfpfpfdhkeyesjyidhghfkpiefpfefxfpktgyfsbkdpdpdpdpdpfeglfycxgwgdfeglgugufdcxgdgmgahffpghfecxgrfehkdpdpdpdpdpbkluehyapa",
        "ur:signing-public-key/tanehskkadlrjkjkisdpjpjkhscxfpfpfpfpfweoglknhsfxehkkiaeyfefpfpfpfpfyfpgyfpfwfpfpfpfwfpgyfyfdgukteygrfliegtfwiyhshfglgoimhfiodliejogsjlgaeyfyeogljnfdingrdlgsingwfwhdkogwjthkfydnksgsgdgakokkhtgwghkpeskshtfldlgsktknfpdyjkgrghghjsgsjnfyemimksgwhkfegufgfgenhkemeyetfwetjnjsenfgglflkkgekkgshgimfeemkneoieiyhkgwkkgtehgrghktgrfxfwiminhfemhtfwgmiyflfphfgljodydyglgogmeciaglhteofykndlgdknjnfwihdyjejpflgtgdeokpdykkgaiyjnfygrhtgufedliyeygrecgdkofygrdyjsfyknguhffdflgugrdyhgkpgokniegdjzidfdhsehdneyecidksfgdlhshsehhsidiyfyemgwiyhsiefegrgdhgihguiekokkhkfeeoeyfeknfygodlimgofdghjnemiajliakkemgmeeghgtesdnfefeiyeekkjzfgidinhkjphsjtjtfpgwfljykpfefgesgyfdenkojokkdyhtjkgagtesksiyjpjeioeejefpioiodnidjkimetgsdlgdeefwjsetguecgdececdlkojsgtisioiehkhtfxiajyjegoksgyeygljkgwkpihhsfyjkiohtihgahfcxiajljnjnihjtjyhpprpfgu",
        "Signature",
    );
}

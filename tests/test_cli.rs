use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::Write;
use std::process::Command; // Run programs
use tempfile::NamedTempFile;

// Text files
#[test]
fn test_text_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;
    let mut persisted_file = file.persist("/tmp/test.txt")?;
    writeln!(
        persisted_file,
        "Lorem ipsum dolor sit amet, \nconsetetur sadipscing 
    elitr, sed diam \nnonumy eirmod tempor invidunt ut labore et dolore 
    magna aliquyam erat, \nsed diam voluptua. At vero eos et accusam et 
    justo duo dolores et ea \nrebum. Stet clita kasd gubergren, no sea 
    takimata sanctus est Lorem \nipsum dolor sit amet. Lorem ipsum 
    dolor sit amet, consetetur sadipscing \nelitr, sed diam nonumy 
    eirmod tempor invidunt ut labore et dolore magna \naliquyam erat, 
    sed diam voluptua. At vero eos et accusam et justo duo \ndolores et 
    ea rebum. Stet clita kasd gubergren, no sea takimata sanctus \nest 
    Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur 
    \nsadipscing elitr, sed diam nonumy eirmod tempor invidunt ut 
    labore et \ndolore magna aliquyam erat, sed diam voluptua. At vero 
    eos et accusam \net justo duo dolores et ea rebum. Stet clita kasd 
    gubergren, no sea \ntakimata sanctus est Lorem ipsum dolor sit 
    amet. \n"
    )?;

    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen").arg("-f").arg("/tmp/test.txt");
    cmd.assert().success().stdout(predicate::str::contains(
        "CTcFSR63wuPKe-CDNDwhyFQWhZi-CR3bGYEViH28H",
    ));

    Ok(())
}

#[test]
fn test_html_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen")
        .arg("-f")
        .arg("./tests/test_data/text/demo.html");
    cmd.assert().success().stdout(predicate::str::contains(
        "CTMjk4o5H96BV-CDagDc9smMbFs-CRFfZgmkBbNRU",
    ));

    Ok(())
}

#[test]
fn test_docx_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen")
        .arg("-f")
        .arg("./tests/test_data/text/demo.docx");
    cmd.assert().success().stdout(predicate::str::contains(
        "CTMjk4o5H96BV-CD6XL9SFyWgsW-CR28vgw3inZGw",
    ));

    Ok(())
}

#[test]
fn test_xlsx_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen")
        .arg("-f")
        .arg("./tests/test_data/text/demo.xlsx");
    cmd.assert().success().stdout(predicate::str::contains(
        "CTcFSR63wuPDc-CDiHCSPHK4xaq-CR5qNj7iLiAnZ",
    ));

    Ok(())
}

//Image files
#[test]
fn test_png_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen")
        .arg("-f")
        .arg("./tests/test_data/image/demo.png");
    cmd.assert().success().stdout(predicate::str::contains(
        "CYDfTq7Qc7Fre-CDij3vGU1BkCZ-CRNssh4Qc1x5B",
    ));

    Ok(())
}

/*#[test]
fn test_jpg_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen")
        .arg("-f")
        .arg("./tests/test_data/image/demo.jpg");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CYDfTq7Qc7Fre-CDYkLqqmQJaQk-CRAPu5NwQgAhv"));

    Ok(())
}*/

#[test]
fn test_gif_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen")
        .arg("-f")
        .arg("./tests/test_data/image/demo.gif");
    cmd.assert().success().stdout(predicate::str::contains(
        "CYDfTq7Qc7Fre-CDbAK5Ut4xC69-CRWT9uvk3PvcB",
    ));

    Ok(())
}

/*#[test]
fn test_tif_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("gen")
        .arg("-f")
        .arg("./tests/test_data/image/demo.tif");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CYDfTq7Qc7Fre-CD29F7ZTmuBGJ-CR5tcBY8co9QS"));

    Ok(())
}*/

#[test]
fn test_batch() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("batch")
        .arg("-r")
        .arg("-d")
        .arg("./tests/test_data");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "CTMjk4o5H96BV-CD6XL9SFyWgsW-CR28vgw3inZGw",
        ))
        .stdout(predicate::str::contains(
            "CYDfTq7Qc7Fre-CDij3vGU1BkCZ-CRNssh4Qc1x5B",
        ));
    Ok(())
}
#[test]
fn test_sim() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("iscc-cli")?;
    cmd.arg("sim")
        .arg("-a")
        .arg("CDcRsq2Wu1x8N")
        .arg("-b")
        .arg("CDij3vGU1BkCZ");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Estimated Similarity: 51.56",
        ));
    Ok(())
}

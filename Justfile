build:
    #/bin/bash
    cd calculator && cargo run ../details.yml ../computed.yml
    cd ../
    typst compile template.typ Reisekostenabrechnung.pdf
    rm computed.yml

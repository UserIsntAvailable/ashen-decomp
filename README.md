# Ashen PackFile unpacker

An unarchiver of an obscure NGage game Ashen

## How to run?

- Place the ROM
    ```bash
    mkdir ./rom/
    cp /path/to/packfile.dat ./rom/packfile.dat
    ```
- Run
    ```bash
    cargo run --release
    ```

## File structure

### Overview

- File is composed out of 3 parts
    - Header
    - File declarations
    - Data
- Some data is compressed using `zlib` algorithm, some isn't
- All data is stored in Little-endian

### Header

| Size (bytes) | Purpose          |
| ------------ | ---------------- |
| `4`          | Signature `PMAN` |
| `4`          | Number of files  |
| `56`         | Copyright        |

### File declarations

- This structure is repeated for every file in the packfile

| Size (bytes) | Purpose                        |
| ------------ | ------------------------------ |
| `4`          | Padding? Always `00 00 00 00`  |
| `4`          | Offset of the file in packfile |
| `4`          | Size of the file               |
| `4`          | Padding? Always `00 00 00 00`  |

### Data

- If data is compressed using zlib

| Size (bytes) | Purpose                |
| ------------ | ---------------------- |
| `2`          | Signature `ZL`         |
| `3`          | Size when uncompressed |
| `*`          | Zlib stream            |

- If data is not compressed, just data stream

### Known file declarations

**⚠️ WARNING ⚠️**

I use the packfile that comes with Ashen 1.06. Your packfile may have different offsets to files, I didn't test with different versions.

| Address (HEX) | Output file  | Purpose                                   |
| ------------- | ------------ | ----------------------------------------- |
| `040`         | `A20.dat`    | **⚠️ ??? ⚠️**                               |
| `050`         | `6F20.dat`   | **⚠️ ??? ⚠️**                               |
| `060`         | `EF20.dat`   | **⚠️ ??? ⚠️**                               |
| `070`         | `16F20.dat`  | **⚠️ ??? ⚠️**                               |
| `080`         | `1EF20.dat`  | **⚠️ ??? ⚠️**                               |
| `090`         | `26F20.dat`  | **⚠️ ??? ⚠️**                               |
| `0A0`         | `2EF20.dat`  | **⚠️ ??? ⚠️**                               |
| `0B0`         | `36F20.dat`  | **⚠️ ??? ⚠️**                               |
| `0C0`         | `3EF20.dat`  | **⚠️ ??? ⚠️**                               |
| `0D0`         | `46F20.dat`  | **⚠️ ??? ⚠️**                               |
| `0E0`         | `4EF20.dat`  | **⚠️ ??? ⚠️**                               |
| `0F0`         | `7DE80.dat`  | **⚠️ ??? ⚠️**                               |
| `100`         | `93CF4.dat`  | **⚠️ ??? ⚠️**                               |
| `110`         | `AE3D0.dat`  | **⚠️ ??? ⚠️**                               |
| `120`         | `D0A90.dat`  | **⚠️ ??? ⚠️**                               |
| `130`         | `EC6BC.dat`  | **⚠️ ??? ⚠️**                               |
| `140`         | `10E58C.dat` | **⚠️ ??? ⚠️**                               |
| `150`         | `127C58.dat` | **⚠️ ??? ⚠️**                               |
| `160`         | `12B2F8.dat` | **⚠️ ??? ⚠️**                               |
| `170`         | `142920.dat` | **⚠️ ??? ⚠️**                               |
| `180`         | `1574A4.dat` | **⚠️ ??? ⚠️**                               |
| `190`         | `166DDC.dat` | **⚠️ ??? ⚠️**                               |
| `1A0`         | `17A464.dat` | **⚠️ ??? ⚠️**                               |
| `1B0`         | `17A688.dat` | **⚠️ ??? ⚠️**                               |
| `1C0`         | `17A7AC.dat` | **⚠️ ??? ⚠️**                               |
| `1D0`         | `17A948.dat` | **⚠️ ??? ⚠️**                               |
| `1E0`         | `17AB00.dat` | **⚠️ ??? ⚠️**                               |
| `1F0`         | `17ACD4.dat` | **⚠️ ??? ⚠️**                               |
| `200`         | `17ADBC.dat` | **⚠️ ??? ⚠️**                               |
| `210`         | `17AEFC.dat` | **⚠️ ??? ⚠️**                               |
| `220`         | `17BA3C.dat` | **⚠️ ??? ⚠️**                               |
| `230`         | `17C574.dat` | **⚠️ ??? ⚠️**                               |
| `240`         | `17D05C.dat` | **⚠️ ??? ⚠️**                               |
| `250`         | `17D388.dat` | **⚠️ ??? ⚠️**                               |
| `260`         | `17FF8C.dat` | **⚠️ ??? ⚠️**                               |
| `270`         | `180664.dat` | **⚠️ ??? ⚠️**                               |
| `280`         | `180D38.dat` | **⚠️ ??? ⚠️**                               |
| `290`         | `1813C4.dat` | **⚠️ ??? ⚠️**                               |
| `2A0`         | `181A24.dat` | **⚠️ ??? ⚠️**                               |
| `2B0`         | `1820B4.dat` | **⚠️ ??? ⚠️**                               |
| `2C0`         | `1827C4.dat` | **⚠️ ??? ⚠️**                               |
| `2D0`         | `182E18.dat` | **⚠️ ??? ⚠️**                               |
| `2E0`         | `1832DC.dat` | **⚠️ ??? ⚠️**                               |
| `2F0`         | `1839F0.dat` | **⚠️ ??? ⚠️**                               |
| `300`         | `184020.dat` | **⚠️ ??? ⚠️**                               |
| `310`         | `18475C.dat` | **⚠️ ??? ⚠️**                               |
| `320`         | `184F4C.dat` | **⚠️ ??? ⚠️**                               |
| `330`         | `1858F0.dat` | **⚠️ ??? ⚠️**                               |
| `340`         | `18602C.dat` | **⚠️ ??? ⚠️**                               |
| `350`         | `1868D4.dat` | **⚠️ ??? ⚠️**                               |
| `360`         | `1871B4.dat` | **⚠️ ??? ⚠️**                               |
| `370`         | `187A4C.dat` | **⚠️ ??? ⚠️**                               |
| `380`         | `1882D0.dat` | **⚠️ ??? ⚠️**                               |
| `390`         | `188B7C.dat` | **⚠️ ??? ⚠️**                               |
| `3A0`         | `189574.dat` | **⚠️ ??? ⚠️**                               |
| `3B0`         | `189AF0.dat` | **⚠️ ??? ⚠️**                               |
| `3C0`         | `18A08C.dat` | **⚠️ ??? ⚠️**                               |
| `3D0`         | `18AB20.dat` | **⚠️ ??? ⚠️**                               |
| `3E0`         | `18B2BC.dat` | **⚠️ ??? ⚠️**                               |
| `3F0`         | `18BB20.dat` | **⚠️ ??? ⚠️**                               |
| `400`         | `18C374.dat` | **⚠️ ??? ⚠️**                               |
| `410`         | `19C57C.dat` | **⚠️ ??? ⚠️**                               |
| `420`         | `1AC784.dat` | **⚠️ ??? ⚠️**                               |
| `430`         | `1BC98C.dat` | **⚠️ ??? ⚠️**                               |
| `440`         | `1CCB94.dat` | **⚠️ ??? ⚠️**                               |
| `450`         | `1DCD9C.dat` | **⚠️ ??? ⚠️**                               |
| `460`         | `1ECFA4.dat` | **⚠️ ??? ⚠️**                               |
| `470`         | `1ED1A4.dat` | **⚠️ ??? ⚠️**                               |
| `480`         | `1ED3A4.dat` | **⚠️ ??? ⚠️**                               |
| `490`         | `1ED5A4.dat` | **⚠️ ??? ⚠️**                               |
| `4A0`         | `1ED7A4.dat` | **⚠️ ??? ⚠️**                               |
| `4B0`         | `1ED9A4.dat` | **⚠️ ??? ⚠️**                               |
| `4C0`         | `1EDBA4.dat` | **⚠️ ??? ⚠️**                               |
| `4D0`         | `1FDBA4.dat` | **⚠️ ??? ⚠️**                               |
| `4E0`         | `20DBA4.dat` | **⚠️ ??? ⚠️**                               |
| `4F0`         | `21DBA4.dat` | **⚠️ ??? ⚠️**                               |
| `500`         | `22DBA4.dat` | Chapter 1.1 geometry                      |
| `510`         | `294D84.dat` | Chapter 1.1 collision                     |
| `520`         | `2A710C.dat` | Chapter 1.1 waypoint                      |
| `530`         | `2A89A4.dat` | Chapter 1.1 color palette                 |
| `540`         | `2B09A4.dat` | Chapter 1.2 geometry                      |
| `550`         | `2E9D88.dat` | Chapter 1.2 collision                     |
| `560`         | `2F56B0.dat` | Chapter 1.2 waypoint                      |
| `570`         | `2F5E40.dat` | Chapter 1.2 color palette                 |
| `580`         | `2FDE40.dat` | Chapter 2.1 geometry                      |
| `590`         | `357648.dat` | Chapter 2.1 collision                     |
| `5A0`         | `36892C.dat` | Chapter 2.1 waypoint                      |
| `5B0`         | `368F28.dat` | Chapter 2.1 color palette                 |
| `5C0`         | `370F28.dat` | Chapter 2.2 geometry                      |
| `5D0`         | `3DF9DC.dat` | Chapter 2.2 collision                     |
| `5E0`         | `3F5974.dat` | Chapter 2.2 waypoint                      |
| `5F0`         | `3F5AFC.dat` | Chapter 2.2 color palette                 |
| `600`         | `3FDAFC.dat` | Chapter 3.1 geometry                      |
| `610`         | `458220.dat` | Chapter 3.1 collision                     |
| `620`         | `469B04.dat` | Chapter 3.1 waypoint                      |
| `630`         | `46A300.dat` | Chapter 3.1 color palette                 |
| `640`         | `472300.dat` | Chapter 3.2 geometry                      |
| `650`         | `4BFCAC.dat` | Chapter 3.2 collision                     |
| `660`         | `4CD11C.dat` | Chapter 3.2 waypoint                      |
| `670`         | `4CD998.dat` | Chapter 3.2 color palette                 |
| `680`         | `4D5998.dat` | Chapter 4.1 geometry                      |
| `690`         | `52EAB4.dat` | Chapter 4.1 collision                     |
| `6A0`         | `53DFAC.dat` | Chapter 4.1 waypoint                      |
| `6B0`         | `53EBA8.dat` | Chapter 4.1 color palette                 |
| `6C0`         | `546BA8.dat` | Chapter 4.2 geometry                      |
| `6D0`         | `58EFC8.dat` | Chapter 4.2 collision                     |
| `6E0`         | `59A818.dat` | Chapter 4.2 waypoint                      |
| `6F0`         | `59B7B8.dat` | Chapter 4.2 color palette                 |
| `700`         | `5A37B8.dat` | Chapter 5.1 geometry                      |
| `710`         | `5F60B8.dat` | Chapter 5.1 collision                     |
| `720`         | `605B3C.dat` | Chapter 5.1 waypoint                      |
| `730`         | `6069B4.dat` | Chapter 5.1 color palette                 |
| `740`         | `60E9B4.dat` | Chapter 5.2 geometry                      |
| `750`         | `683FD4.dat` | Chapter 5.2 collision                     |
| `760`         | `698898.dat` | Chapter 5.2 waypoint                      |
| `770`         | `69A6A0.dat` | Chapter 5.2 color palette                 |
| `780`         | `6A26A0.dat` | Chapter 6.1 geometry                      |
| `790`         | `6F23F4.dat` | Chapter 6.1 collision                     |
| `7A0`         | `6FF774.dat` | Chapter 6.1 waypoint                      |
| `7B0`         | `6FFAA0.dat` | Chapter 6.1 color palette                 |
| `7C0`         | `707AA0.dat` | Chapter 7.1 geometry                      |
| `7D0`         | `775A2C.dat` | Chapter 7.1 collision                     |
| `7E0`         | `7882C0.dat` | Chapter 7.1 waypoint                      |
| `7F0`         | `788D48.dat` | Chapter 7.1 color palette                 |
| `800`         | `790D48.dat` | Chapter 8.1 geometry                      |
| `810`         | `7E3990.dat` | Chapter 8.1 collision                     |
| `820`         | `7F5028.dat` | Chapter 8.1 waypoint                      |
| `830`         | `7F5464.dat` | Chapter 8.1 color palette                 |
| `840`         | `7FD464.dat` | Deathmatch 1 geometry                     |
| `850`         | `823F54.dat` | Deathmatch 1 collision                    |
| `860`         | `82A85C.dat` | Deathmatch 1 color palette                |
| `870`         | `83285C.dat` | Deathmatch 2 geometry                     |
| `880`         | `84F7D0.dat` | Deathmatch 2 collision                    |
| `890`         | `856108.dat` | Deathmatch 2 color palette                |
| `8A0`         | `85E108.dat` | Deathmatch 3 geometry                     |
| `8B0`         | `870414.dat` | Deathmatch 3 collision                    |
| `8C0`         | `87352C.dat` | Deathmatch 3 color palette                |
| `8D0`         | `87B52C.dat` | Deathmatch 4 geometry                     |
| `8E0`         | `8A8F2C.dat` | Deathmatch 4 collision                    |
| `8F0`         | `8B2508.dat` | Deathmatch 4 color palette                |
| `900`         | `8BA508.dat` | 💡 Test level for enemies geometry        |
| `910`         | `8C4188.dat` | 💡 Test level for enemies collision       |
| `920`         | `8C4F64.dat` | **💡⚠️ Test level for enemies waypoint ⚠️** |
| `930`         | `8C4F84.dat` | 💡 Test level ??? geometry                |
| `940`         | `8CFF88.dat` | **💡⚠️ Test level ??? collision ⚠️**        |
| `950`         | `8D1000.dat` | **💡⚠️ Test level ??? waypoint ⚠️**         |
| `960`         | `8D102C.dat` | **⚠️ ??? ⚠️**                               |
| `970`         | `8DB36C.dat` | **⚠️ ??? ⚠️**                               |
| `980`         | `8DCC74.dat` | **⚠️ ??? ⚠️**                               |
| `990`         | `8E160C.dat` | **⚠️ ??? ⚠️**                               |
| `9A0`         | `A25878.dat` | **⚠️ ??? ⚠️**                               |
| `9B0`         | `BBC974.dat` | **⚠️ ??? ⚠️**                               |
| `9C0`         | `D9924C.dat` | Text - English UK                         |
| `9D0`         | `D9B808.dat` | Text - English US                         |
| `9E0`         | `D9DDC4.dat` | Text - French                             |
| `9F0`         | `DA0904.dat` | Text - Italian                            |
| `A00`         | `DA34B8.dat` | Text - German                             |
| `A10`         | `DA5B9C.dat` | Text - Spanish                            |

### Known file formats

### Text bank

- Starts with `9B 01 00 00`
- The rest is encoded in UTF16
- All text chunks are separated with `\r\r`
- There are weird characters like `20 20` which are probably to control in-game scripts

## Discoveries

- Test level for enemies
    - There is a pool with fish enemies, which I don't remember seeing in the game
- Test level ???
    - Collision by itself works with other level geometry, geometry by itself also works, but not together


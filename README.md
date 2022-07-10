

- 마이그레이션
DATABASE_URL="mysql://root:group@localhost:3306/group" sea-orm-cli migrate refresh

- DB스키마로 마이그레이션 파일 만들기(Generate Entity from Database)
sea-orm-cli generate entity -u mysql://root:group@localhost:3306/group -o entity/src/
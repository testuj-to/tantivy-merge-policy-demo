const fs = require("fs");
const { randomUUID } = require("crypto");
const minimist = require("minimist");
const {
    fakerEN_GB,
    fakerEN_US,
    fakerCS_CZ,
    fakerFR,
    fakerDE,
} = require("@faker-js/faker");

const { count, output } = minimist(process.argv.slice(2), {
    string: ["count", "output"],
});

const _count = Number(count);
if (isNaN(_count)) {
    throw new Error("Invalid 'count': expected positive integer");
}

const locales = [
    "en_gb",
    "en_us",
    "cs_cz",
    "fr",
    "de",
];

const generatePerson = () => {
    const locale = locales[Math.floor(Math.random() * locales.length)];
    let country;

    let faker = fakerEN_GB;
    switch (locale) {
    case "en_gb":
        faker = fakerEN_GB;
        country = "gb";
        break;
    case "en_us":
        faker = fakerEN_US;
        country = "us";
        break;
    case "cs_cz":
        faker = fakerCS_CZ;
        country = "cz";
        break;
    case "fr":
        faker = fakerFR;
        country = "fr";
        break;
    case "de":
        faker = fakerDE;
        country = "de";
        break;
    }

    const id = randomUUID();
    const sex = fakerEN_GB.person.sex();
    const firstName = faker.person.firstName(sex);
    const lastName = faker.person.lastName(sex);
    const email = faker.internet.email({ firstName, lastName });

    const address = {
        country,
        zipCode: faker.location.zipCode(),
        city: faker.location.city(),
        line1: faker.location.streetAddress(),
    };

    return {
        id,
        firstName,
        lastName,
        email,
        sex,
        address,
        settings: {
            locale,
        },
    };
};

const main = async() => {
    const people = [];

    for (let index = 0; index < _count; index += 1) {
        people.push(generatePerson());
    }

    fs.writeFileSync(output, JSON.stringify(people));
};

main();

-- Your SQL goes here
CREATE TABLE persons(
    id INT GENERATED ALWAYS AS IDENTITY,
    name VARCHAR NOT NULL,
    PRIMARY KEY(id),
    bankAccount VARCHAR
);

CREATE TABLE birthdays(
    id INT GENERATED ALWAYS AS IDENTITY,
    date DATE NOT NULL,
    PRIMARY KEY(id),
    person_id INT,
    CONSTRAINT fk_person
        FOREIGN KEY(person_id)
            REFERENCES persons(id)
);

CREATE TABLE parties(
    id INT GENERATED ALWAYS AS IDENTITY,
    date DATE NOT NULL,
    PRIMARY KEY(id),
    host_id INT,
    name VARCHAR NOT NULL,
    birthday_id INT,

    CONSTRAINT fk_host
        FOREIGN KEY(host_id)
            REFERENCES persons(id),
    CONSTRAINT fk_birthday
        FOREIGN KEY(birthday_id)
            REFERENCES birthdays(id)
);

CREATE TABLE party_guests(
    party_id INT REFERENCES parties(id) ON UPDATE CASCADE ON DELETE CASCADE,
    guest_id INT REFERENCES persons(id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT party_guests_pk PRIMARY KEY (party_id, guest_id)
);

CREATE TABLE gifts(
    id INT GENERATED ALWAYS AS IDENTITY,
    PRIMARY KEY(ID),
    name VARCHAR NOT NULL,
    price MONEY,
    birthday_id INT,
    CONSTRAINT fk_birthday
        FOREIGN KEY(birthday_id)
            REFERENCES birthdays(id)
);

CREATE TABLE gift_givers(
    gift_id INT REFERENCES gifts(id) ON UPDATE CASCADE ON DELETE CASCADE,
    giver_id INT REFERENCES persons(id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT gift_givers_pk PRIMARY KEY (gift_id, giver_id)
);

CREATE TABLE shopping_carts(
    id INT GENERATED ALWAYS AS IDENTITY,
    PRIMARY KEY(ID),
    owner_id INT,
    CONSTRAINT fk_owner
        FOREIGN KEY(owner_id)
            REFERENCES persons(id)
);

CREATE TABLE payments(
    id INT GENERATED ALWAYS AS IDENTITY,
    PRIMARY KEY(ID),
    price MONEY,
    vindicator_id INT,
    vindicatee_id INT,
    gift_id INT,
    shopping_cart_id INT,
    CONSTRAINT fk_vindicator
        FOREIGN KEY(vindicator_id)
            REFERENCES persons(id),
    CONSTRAINT fk_vindicatee
        FOREIGN KEY(vindicatee_id)
            REFERENCES persons(id),
    CONSTRAINT fk_gift
        FOREIGN KEY(gift_id)
            REFERENCES gifts(id),
    CONSTRAINT fk_shopping_cart
        FOREIGN KEY(shopping_cart_id)
            REFERENCES shopping_carts(id)
);

CREATE TABLE products(
    id INT GENERATED ALWAYS AS IDENTITY,
    PRIMARY KEY(ID),
    price MONEY,
    quantity INT,
    shopping_cart_id INT,
    CONSTRAINT fk_shopping_cart
        FOREIGN KEY(shopping_cart_id)
            REFERENCES shopping_carts(id)
);

CREATE TABLE product_consumers(
    product_id INT REFERENCES products(id) ON UPDATE CASCADE ON DELETE CASCADE,
    consumer_id INT REFERENCES persons(id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT product_consumers_pk PRIMARY KEY (product_id, consumer_id)
);
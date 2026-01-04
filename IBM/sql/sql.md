## Databases and SQL for Data Science with Python

### Basic SQL

- The Data Manipulation Language (DML) statements read and modify data.
- The search condition of the WHERE clause uses a predicate to refine the search.
- The SQL retrieves specific data from databases.
- The COUNT, DISTINCT, and LIMIT are expressions used with SELECT statements.
- The real-world applications of SELECT statements.
- The INSERT, UPDATE, and DELETE are DML statements for populating and changing tables.

| Command | Syntax | Description | Example |
|---------|--------|-------------|---------|
| SELECT | `SELECT column1, column2, ... FROM table_name;` | SELECT statement is used to fetch data from a database. | `SELECT city FROM placeofinterest;` |
| WHERE | `SELECT column1, column2, ... FROM table_name WHERE condition;` | WHERE clause is used to extract only those records that fulfill a specified condition. | `SELECT * FROM placeofinterest WHERE city = 'Rome';` |
| COUNT | `SELECT COUNT(*) FROM table_name;` | COUNT is a function that counts the number of rows where the column is not NULL. | `SELECT COUNT(country) FROM placeofinterest WHERE country = 'Canada';` |
| DISTINCT | `SELECT DISTINCT columnname FROM table_name;` | DISTINCT returns unique values in specified columns. | `SELECT DISTINCT country FROM placeofinterest WHERE type = 'historical';` |
| LIMIT | `SELECT * FROM table_name LIMIT number;` | LIMIT specifies the maximum number of rows in the result. | `SELECT * FROM placeofinterest WHERE airport = 'Pearson' LIMIT 5;` |
| INSERT | `INSERT INTO table_name (column1, column2, ...) VALUES (value1, value2, ...);` | INSERT adds new rows to a table. | `INSERT INTO placeofinterest (name, type, city, country, airport) VALUES ('Niagara Waterfalls', 'Nature', 'Toronto', 'Canada', 'Pearson');` |
| UPDATE | `UPDATE table_name SET column1 = value1 WHERE condition;` | UPDATE modifies existing rows in the table. | `UPDATE placeofinterest SET name = 'Niagara Falls' WHERE name = 'Niagara Waterfalls';` |
| DELETE | `DELETE FROM table_name WHERE condition;` | DELETE removes rows matching the condition from the table. | `DELETE FROM placeofinterest WHERE city IN ('Rome', 'Vienna');` |

### Relational Database Concepts and Tables

- A database is a repository of data that provides functionality for adding, modifying, and querying the data.
- SQL is a language used to query or retrieve data from a relational database.
- The Relational Model is the most used data model for databases because it allows for data independence.
- The primary key of a relational table uniquely identifies each tuple or row, preventing duplication of data and providing a way of defining relationships between tables.
- SQL statements fall into two different categories: Data Definition Language (DDL) statements and Data Manipulation Language (DML) statements.

| Command                       | Syntax | Description | Example |
|------------------------------|--------|-------------|---------|
| CREATE TABLE | MySQL/DB2: `CREATE TABLE table_name (col1 datatype optional keyword, col2 datatype optional keyword, ..., coln datatype optional keyword)` | Used to create a table. Each column must have a name, data type, and optional keywords like `PRIMARY KEY`, `NOT NULL`, etc. | `CREATE TABLE employee ( employee_id CHAR(2) PRIMARY KEY, first_name VARCHAR(30) NOT NULL, mobile INT );` |
| ALTER TABLE – ADD COLUMN | MySQL/DB2 Option 1: `ALTER TABLE table_name ADD column_name_1 datatype, ADD column_name_n datatype;`<br>Option 2: `ALTER TABLE table_name ADD COLUMN column_name_1 datatype, ADD COLUMN column_name_n datatype;` | Adds new column(s) to an existing table. Both `ADD` and `ADD COLUMN` are valid syntax. | `ALTER TABLE employee ADD income BIGINT;`<br>`ALTER TABLE employee ADD COLUMN income BIGINT;` |
| ALTER TABLE – ALTER COLUMN | MySQL: `ALTER TABLE table_name MODIFY column_name new_data_type;`<br>DB2: `ALTER TABLE table_name ALTER COLUMN column_name SET DATA TYPE datatype;` | Modifies the data type of existing columns. Syntax differs between MySQL and DB2. | MySQL: `ALTER TABLE employee MODIFY mobile CHAR(20);`<br>DB2: `ALTER TABLE employee ALTER COLUMN mobile SET DATA TYPE CHAR(20);` |
| ALTER TABLE – DROP COLUMN | MySQL/DB2: `ALTER TABLE table_name DROP COLUMN column_name;` | Removes a column from an existing table. | `ALTER TABLE employee DROP COLUMN mobile;` |
| ALTER TABLE – RENAME COLUMN | MySQL: `ALTER TABLE table_name CHANGE COLUMN old_name new_name datatype;`<br>DB2: `ALTER TABLE table_name RENAME COLUMN old_name TO new_name;` | Renames a column. MySQL uses `CHANGE COLUMN`, DB2 uses `RENAME COLUMN`. | MySQL: `ALTER TABLE employee CHANGE COLUMN first_name name VARCHAR(255);`<br>DB2: `ALTER TABLE employee RENAME COLUMN first_name TO name;` |
| TRUNCATE TABLE | MySQL: `TRUNCATE TABLE table_name;`<br>DB2: `TRUNCATE TABLE table_name IMMEDIATE;` | Deletes **all rows** from a table. Cannot be undone in DB2 when using `IMMEDIATE`. | MySQL: `TRUNCATE TABLE employee;`<br>DB2: `TRUNCATE TABLE employee IMMEDIATE;` |
| DROP TABLE | MySQL/DB2: `DROP TABLE table_name;` | Deletes the entire table and all its data permanently. | `DROP TABLE employee;` |

### Refining Your Results

- You can use the WHERE clause to refine your query results.
- The search condition of the WHERE clause uses a predicate to refine the search.
- You can use the wildcard character (%) as a substitute for unknown characters in a pattern.
- You can use BETWEEN ... AND ... to specify a range of numbers.
- You can sort query results into ascending or descending order, using the ORDER BY clause to specify the column to sort on.
- You can group query results by using the GROUP BY clause.

| Command    | Syntax (MySQL/DB2)| Description | Example (MySQL/DB2)  |
|------------|---|--|------|
| LIKE       | `SELECT column1, column2, ... FROM table_name WHERE columnN LIKE pattern;`  | `LIKE` is used in a `WHERE` clause to search for a specified pattern. Wildcards: `%` (any chars), `_` (single char).   | `SELECT f_name, l_name FROM employees WHERE address LIKE '%Elgin,IL%';` – returns entries with "Elgin,IL" in the address. |
| BETWEEN    | `SELECT column_name(s) FROM table_name WHERE column_name BETWEEN value1 AND value2;` | `BETWEEN` selects values within a range (inclusive). Can be used with numbers, text, or dates.  | `SELECT * FROM employees WHERE salary BETWEEN 40000 AND 80000;` – selects employees earning between 40,000 and 80,000. |
| ORDER BY   | `SELECT column1, column2, ... FROM table_name ORDER BY column1, column2, ... ASC|DESC;` | `ORDER BY` sorts the result set. Default is ascending. Multiple columns are sorted in order of appearance.  | `SELECT f_name, l_name, dep_id FROM employees ORDER BY dep_id DESC, l_name;` – sorts by department ID descending, then by last name ascending. |
| GROUP BY   | `SELECT column_name(s) FROM table_name GROUP BY column_name(s)` | `GROUP BY` groups rows that have the same values into summary rows (e.g., `COUNT()`, `AVG()`, etc.). | `SELECT dep_id, COUNT(*) FROM employees GROUP BY dep_id;` – returns department IDs and the number of employees in each. |
| HAVING     | `SELECT column_name(s) FROM table_name GROUP BY column_name(s) HAVING condition` | `HAVING` filters group results after `GROUP BY`. Use it when filtering on aggregate functions (`COUNT`, `AVG`, etc.), which can't be used with `WHERE`. | `SELECT dep_id, COUNT(*) AS "NUM_EMPLOYEES", AVG(salary) AS "AVG_SALARY" FROM employees GROUP BY dep_id HAVING COUNT(*) < 4 ORDER BY AVG_SALARY;` – shows departments with fewer than 4 employees, sorted by average salary.  |

### Functions, Multiple Tables, and Sub-queries

- Tools for database management that offer built-in functions for performing operations on data within the database itself.
- That when working with large datasets, you may save time by using built-in functions rather than first retrieving the data into your application and then executing functions on the retrieved data.
- You can use sub-queries to form more powerful queries than otherwise.
- You can use a sub-select expression to evaluate some built-in aggregate functions like the average function.
- Derived tables or table expressions are sub-queries where the outer query uses the results of the sub-query as a data source.

|Command|Syntax (MySQL/DB2)|Description|Example (MySQL/DB2)|
|-|-|-|-|
|COUNT|SELECT COUNT(column_name) FROM table_name WHERE condition;|Returns number of rows matching a criterion.|SELECT COUNT(dep_id) FROM employees;|
|AVG|SELECT AVG(column_name) FROM table_name WHERE condition;|Returns average value of a numeric column.|SELECT AVG(salary) FROM employees;|
|SUM|SELECT SUM(column_name) FROM table_name WHERE condition;|Returns total sum of a numeric column.|SELECT SUM(salary) FROM employees;|
|MIN|SELECT MIN(column_name) FROM table_name WHERE condition;|Returns smallest value of a column.|SELECT MIN(salary) FROM employees;|
|MAX|SELECT MAX(column_name) FROM table_name WHERE condition;|Returns largest value of a column.|SELECT MAX(salary) FROM employees;|
|ROUND|SELECT ROUND(number,decimals);|Rounds a number to given decimals.|SELECT ROUND(salary,2) FROM employees;|
|LENGTH|SELECT LENGTH(column_name) FROM table;|Returns string length in bytes.|SELECT LENGTH(f_name) FROM employees;|
|UCASE|SELECT UCASE(column_name) FROM table;|Converts text to uppercase.|SELECT UCASE(f_name) FROM employees;|
|LCASE|SELECT LCASE(column_name) FROM table;|Converts text to lowercase.|SELECT LCASE(f_name) FROM employees;|
|DISTINCT|SELECT DISTINCT column_name FROM table;|Removes duplicate values.|SELECT DISTINCT UCASE(f_name) FROM employees;|
|DAY|SELECT DAY(column_name) FROM table;|Returns day of month from date.|SELECT DAY(b_date) FROM employees WHERE emp_id='E1002';|
|CURRENT_DATE|SELECT CURRENT_DATE;|Returns current date.|SELECT CURRENT_DATE;|
|DATEDIFF|SELECT DATEDIFF(date1,date2);|Returns difference in days between dates.|SELECT DATEDIFF(CURRENT_DATE,date_column) FROM table;|
|FROM_DAYS|SELECT FROM_DAYS(number_of_days);|Converts days to date format.|SELECT FROM_DAYS(DATEDIFF(CURRENT_DATE,date_column)) FROM table;|
|DATE_ADD|SELECT DATE_ADD(date,INTERVAL n type);|Adds interval to a date.|SELECT DATE_ADD(date,INTERVAL 3 DAY);|
|DATE_SUB|SELECT DATE_SUB(date,INTERVAL n type);|Subtracts interval from a date.|SELECT DATE_SUB(date,INTERVAL 3 DAY);|
|Subquery|SELECT column FROM table WHERE column OPERATOR (SELECT column FROM table);|Query nested inside another query.|SELECT emp_id FROM employees WHERE salary<(SELECT AVG(salary) FROM employees);|
|Implicit Inner Join|SELECT columns FROM table1,table2 WHERE table1.col=table2.col;|Returns matching rows from both tables.|SELECT * FROM employees,jobs WHERE employees.job_id=jobs.job_ident;|
|Implicit Cross Join|SELECT columns FROM table1,table2;|Returns Cartesian product of tables.|SELECT * FROM employees,jobs;|

### Accessing databases using Python

- Magic commands are special commands that provide special functionalities.
- Cell magics are commands prefixed with two %% characters and operate on multiple input lines.
- DB APIs are commands prefixed with two %% characters and operate on multiple input lines.
- The two main concepts in the Python DB API are Connection Objects and Query Objects.
- A database cursor is a control structure that enables traversal over the records in a database.
- Pandas’ methods are equipped with common mathematical and statistical methods.  
- The pandas.read_csv() function is used to read the database CSV file.
- The sqlite3.connect() function is used to connect to a database.
- To use pandas to retrieve data from the database tables, load data using the read_sql method and select the SQL Select Query
- A categorical scatterplot is created using the swarmplot() method by the seaborn package.

### Views, Stored Procedures, and Transactions

- Views are a dynamic mechanism for presenting data from one or more tables.A transaction represents a complete unit of work, which can be one or more SQL statements.
- An ACID transaction is one where all the SQL statements must complete successfully, or none at all.
- A stored procedure is a set of SQL statements that are stored and executed on the database server, allowing you to send one statement as an alternative to sending multiple statements.
- You can write stored procedures in many different languages like SQL PL, PL/SQL, Java, and C.

### JOIN Statements

- A join combines the rows from two or more tables based on a relationship between certain columns in these tables.
- To combine data from three or more different tables, you simply add new joins to the SQL statement.
- There are two types of joins: inner join and outer join; and three types of outer joins: left outer join, right outer join, and full outer join.
- The most common type of join is the inner join, which matches the results from two tables and returns the selected elements from each table, only where corresponding elements in both the tables are the same.
- You can use an alias as shorthand for a table or column name.

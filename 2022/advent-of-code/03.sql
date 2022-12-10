
CREATE TABLE rucksacks (contents TEXT);
.import --csv '03.in' rucksacks

CREATE TABLE contents (sack_id INTEGER, compartment INTEGER, item CHAR);

WITH RECURSIVE split(sack, ch, n, rest) AS (
    SELECT ROWID, '', 1, substr(contents, 1, length(contents)/2) FROM rucksacks
  UNION
    SELECT ROWID, '', 2, substr(contents, 1+length(contents)/2) FROM rucksacks
  UNION ALL
    SELECT sack, substr(rest, 1, 1), n, substr(rest, 2)
    FROM split
    WHERE rest <> ''
)
INSERT INTO contents
SELECT sack, n, ch
  FROM split
 WHERE ch != ''
 GROUP BY sack, n, ch
;


SELECT SUM(CASE WHEN unicode(item) > 96 THEN unicode(item)-96 ELSE unicode(item)-38 END) AS "Part 1"
  FROM (
    SELECT sack_id, item
      FROM contents AS c1
     WHERE compartment = 1
       AND item IN (SELECT item FROM contents AS c2 WHERE c2.sack_id = c1.sack_id AND c2.compartment = 2)
);


SELECT SUM(CASE WHEN unicode(item) > 96 THEN unicode(item)-96 ELSE unicode(item)-38 END) AS "Part 2"
  FROM (
    SELECT grp, item, SUM(fnd)
      FROM (
        SELECT sack_id, (c.sack_id - 1)/3 as grp, item, instr(contents, item) > 0 as fnd
          FROM contents AS c LEFT JOIN rucksacks AS r
         WHERE ((c.sack_id - 1)/3) = ((r.ROWID - 1)/3)
         )
    GROUP BY grp, item
    HAVING SUM(fnd) >= 9
)
;

CREATE VIRTUAL TABLE name_fts USING fts5(name, macro_id UNINDEXED, alias_id UNINDEXED);

-- Triggers to keep the FTS index up to date.
CREATE TRIGGER fts_alias_insert AFTER INSERT ON alias BEGIN
 INSERT INTO name_fts(rowid, name, macro_id, alias_id) VALUES (new.name, new.macro_id, new.id);
END;
CREATE TRIGGER fts_alias_delete AFTER DELETE ON alias BEGIN
 INSERT INTO name_fts(name_fts, rowid, name, macro_id, alias_id) VALUES('delete', old.name, old.macro_id, old.id);
END;
CREATE TRIGGER fts_alias_update AFTER UPDATE ON alias BEGIN
 INSERT INTO name_fts(name_fts, rowid, name, macro_id, alias_id) VALUES('delete', old.name, old.macro_id, old.id);
 INSERT INTO name_fts(rowid, name, macro_id, alias_id) VALUES (new.name, new.macro_id, new.id);
END;

CREATE TRIGGER fts_macro_insert AFTER INSERT ON macro BEGIN
 INSERT INTO name_fts(rowid, name, macro_id) VALUES (new.name,  new.id);
END;
CREATE TRIGGER fts_macro_delete AFTER DELETE ON macro BEGIN
 INSERT INTO name_fts(name_fts, rowid, name, macro_id) VALUES('delete', old.name, old.id);
END;
CREATE TRIGGER fts_macro_update AFTER UPDATE ON macro BEGIN
 INSERT INTO name_fts(name_fts, rowid, name, macro_id) VALUES('delete', old.name, old.id);
 INSERT INTO name_fts(rowid, name, macro_id) VALUES (new.name, new.id);
END;
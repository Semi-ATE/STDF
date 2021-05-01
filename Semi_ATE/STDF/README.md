ToDo:
# library
- [x] split records.py in individual modules (same name as the records)
- [ ] surrond self.fields with `# fmt: off` and `# fmt: on` (prepare for [Black](https://github.com/psf/black))
- [ ] add a check for python <= 3.7
- [ ] move support functions to __init__.py
- [x] records.py should be empty now, so delete it
- [ ] create a list of all used types (combinations), and remove anything else from STDR (cfr also tests)
- [x] remove all V3 records.
- [ ] (issue #23) implement JSON support (import/export) after agreement how data should look like in the JSON
- [ ] find the memory extension spec on internet and add it to the docs (we lost it along the road)
- [ ] implement the output functions as follows:
      - __repr__ --> based on a call to the record class with a bytes record
      - __str__ --> human readable version (used with print)
      - __bytes__ --> output the STDF record (=bytes type)
      - to_json --> output the json equivalent of the record (=bytes type, starting with a '{')
      - to_ATDF --> output a string (utf-8 encoded) that is the ATDF version of the record
- [ ] implement the __call__ further so it takes:
      - a bytes type :
        - if the byte array starts with a '{' (and ends with a '}'?) it is a JSON
        - if not, it is an STDF
      - a string type : it is an ATDF
- [ ] move `self.info` to docstring of the class (info availabe in editors)
- [ ] implement 'FPE' for all applicable records (cfr PTR)
- [ ] re-enable the DT !!! (I didn't put it there for nothing!!!)
- [ ] re-enable the magic numbers !!! (I didn't put it there for nothing!!!)
- [ ] The comment out class File from utils.py was renamed to STDFFile and moved to STDFFile file. Need to be checked if works.

# tests
- [ ] `STDF` needs heavy testing, at least 3 tests (smaller than min, bigger than max, nominal) per STDF-type.
- [ ] all non-STDR records need just 3 test:
      - one that checks that to_json & reinstantination with that equals the original
      - one that checks that to_ATDF & reinstantination with that equals the original
- [x] test that reinstantination with __repr__ equals the original
- [x] test that the 3 supported zippings work
- [x] add tests for ATDF
- [ ] add tests for JSON

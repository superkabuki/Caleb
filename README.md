# Caleb
The Caleb files

--------------------------------

CALEB: Yeah, those first tests seem useless. How thorough do you want these tests? Decode a byte string with `cue`
and check that all the fields are correct? What sort of bugs have you encountered after building?

[pytest](https://docs.pytest.org/en/stable/) seems like a good tool, though I've never used it. The docs for it look
good too. Make a `./threefive3/tests/` folder to put tests in?

------------------------------

## Speaking of testing, I think this kind of unit test is kind of silly

```py3
    def test_equality(self):
        t1 = otio.opentime.RationalTime(30.2)
        self.assertEqual(t1, t1)
        t2 = otio.opentime.RationalTime(30.2)
        self.assertTrue(t1 is not t2)
        self.assertEqual(t1, t2)
        t3 = otio.opentime.RationalTime(60.4, 2.0)
        self.assertEqual(t1, t3)

    def test_inequality(self):
        t1 = otio.opentime.RationalTime(30.2)
        self.assertEqual(t1, t1)
        t2 = otio.opentime.RationalTime(33.2)
        self.assertTrue(t1 is not t2)
        self.assertNotEqual(t1, t2)
        t3 = otio.opentime.RationalTime(30.2)
        self.assertTrue(t1 is not t3)
        self.assertFalse(t1 != t3)
```


I want to do much higher level testing like:
```py3
from threefive3.crc import crc32
aic = crc32(b'AdrianIsCool')
aic == 4107250885
```
and mostly with the Cue class decoding SCTE-35.






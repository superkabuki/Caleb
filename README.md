# Caleb
The Caleb files

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





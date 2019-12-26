#ifndef Z2UTIL_MUSIC_MUSIC
#define Z2UTIL_MUSIC_MUSIC

#include <fstream>
#include <string>
#include <unordered_map>
#include <vector>

namespace z2music {

// Forward declaration since other classes refer to rom.
class Rom;

class Note {
  public:
    enum class Duration {
      Sixteenth      = 0x00,
      DottedQuarter  = 0x01,
      DottedEighth   = 0x40,
      Half           = 0x41,
      Eighth         = 0x80,
      EighthTriplet  = 0x81,
      Quarter        = 0xc0,
      QuarterTriplet = 0xc1,
    };

    enum class Pitch {
                   Rest = 0x02, E3  = 0x04, G3  = 0x06,
      Gs3  = 0x08, A3   = 0x0a, As3 = 0x0c, B3  = 0x0e,
      Ab3  = 0x08,              Bb3 = 0x0c,
      C4   = 0x10, Cs4  = 0x12, D4  = 0x14, Ds4 = 0x16,
                   Db4  = 0x12,             Eb4 = 0x16,
      E4   = 0x18, F4   = 0x1a, Fs4 = 0x1c, G4  = 0x1e,
                                Gb4 = 0x1c,
      Gs4  = 0x20, A4   = 0x22, As4 = 0x24, B4  = 0x26,
      Ab4  = 0x20,              Bb4 = 0x24,
      C5   = 0x28, Cs5  = 0x2a, D5  = 0x2c, Ds5 = 0x2e,
                   Db5  = 0x2a,             Eb5 = 0x2e,
      E5   = 0x30, F5   = 0x32, Fs5 = 0x34, G5  = 0x36,
                                Gb5 = 0x34,
      A5   = 0x38, As5  = 0x3a, B5  = 0x3c, C6  = 0x3e,
                   Bb5  = 0x3a,
    };

    Note(uint8_t value);
    Note(Duration d, Pitch p);

    Duration duration() const;
    Pitch pitch() const;

    void duration(Duration d);
    void pitch(Pitch p);

    size_t length() const;
    std::string pitch_string() const;

    uint8_t encode() const;

  private:
    uint8_t value_;
};

class Pattern {
  public:
    enum class Channel { Pulse1, Pulse2, Triangle, Noise };

    Pattern();
    Pattern(const Rom& rom, size_t address);

    size_t length() const;

    void add_notes(Channel ch, std::initializer_list<Note> notes);
    void clear();
    std::vector<Note> notes(Channel ch) const;

    // TODO figure out if the tempo values are meaningful
    void tempo(uint8_t tempo);
    uint8_t tempo() const;

    bool validate() const;

    std::vector<uint8_t> note_data() const;
    std::vector<uint8_t> meta_data(size_t pw1_address) const;

  private:
    uint8_t tempo_;
    std::unordered_map<Channel, std::vector<Note>> notes_;

    void read_notes(Channel ch, const Rom& rom, size_t address);
};

class Song {
  public:
    Song();
    Song(const Rom& rom, size_t address, size_t entry);

    void add_pattern(const Pattern& pattern);
    void set_sequence(const std::vector<size_t>& seq);
    void append_sequence(size_t n);

    std::vector<uint8_t> sequence_data(uint8_t first) const;

    size_t sequence_length() const;
    size_t pattern_count() const;
    size_t metadata_length() const;

    std::vector<Pattern> patterns();

    Pattern* at(size_t i);
    const Pattern* at(size_t i) const;

  private:
    std::vector<Pattern> patterns_;
    std::vector<size_t> sequence_;
};

class Rom {
  public:
    enum class SongTitle {
      OverworldIntro, OverworldTheme,
      BattleTheme,
      CaveItemFanfare,

      TownIntro, TownTheme,
      HouseTheme,
      TownItemFanfare,

      PalaceIntro, PalaceTheme,
      BossTheme,
      PalaceItemFanfare,
      CrystalFanfare,

      GreatPalaceIntro, GreatPalaceTheme,
      ZeldaTheme,
      CreditsTheme,
      GreatPalaceItemFanfare,
      TriforceFanfare,
      FinalBossTheme,
    };

    Rom(const std::string& filename);

    uint8_t getc(size_t address) const;
    void putc(size_t address, uint8_t data);

    void read(uint8_t* buffer, size_t address, size_t length) const;
    void write(size_t address, std::vector<uint8_t> data);

    bool commit();
    void save(const std::string& filename);

    Song* song(SongTitle title);

  private:
    static constexpr size_t kHeaderSize =     0x10;
    static constexpr size_t kRomSize    = 0x040000;

    static constexpr size_t kOverworldSongTable   = 0x01a000;
    static constexpr size_t kTownSongTable        = 0x01a3ca;
    static constexpr size_t kPalaceSongTable      = 0x01a62f;
    static constexpr size_t kGreatPalaceSongTable = 0x01a936;

    uint8_t header_[kHeaderSize];
    uint8_t data_[kRomSize];

    std::unordered_map<SongTitle, Song> songs_;

    void commit(size_t address, std::initializer_list<SongTitle> songs);
    size_t metadata_length(std::vector<SongTitle> songs);
};

} // namespace z2music

#endif // define Z2UTIL_MUSIC_MUSIC

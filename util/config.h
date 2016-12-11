#ifndef UTIL_CONFIG_H
#define UTIL_CONFIG_H
#include <string>
#include <functional>

#include "google/protobuf/text_format.h"
#include "util/file.h"
#include "util/logging.h"
#include "util/os.h"

template<typename T>
class ConfigLoader {
  public:
    static ConfigLoader* Get() {
        static ConfigLoader<T> singleton;
        return &singleton;
    }
    static inline const T& GetConfig() { return Get()->config_; }
    static inline T* MutableConfig() { return &Get()->config_; }

    void Load(const std::string& filename,
              std::function<void(T*)> postprocess=nullptr) {
        Load(filename, &config_);
        if (postprocess)
            postprocess(&config_);
    }
    void Load(const std::string& filename, T* config) {
        std::string pb;
        std::string path = File::Dirname(filename);
        T local_config;

        if (!File::GetContents(filename, &pb)) {
            LOG(FATAL, "Could not read '", filename, "'.");
        }
        if (!google::protobuf::TextFormat::ParseFromString(pb, &local_config)) {
            LOG(FATAL, "Could not parse '", filename, "'.");
        }

        for(const auto& file : local_config.load()) {
            Load(os::path::Join({path, file}), &local_config);
        }

        config->MergeFrom(local_config);
    }
    inline const T& config() const { return config_; }
  protected:
    ConfigLoader() {};
    T config_;
};

#endif // UTIL_CONFIG_H

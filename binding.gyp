{
    'targets': [
        {
            'target_name': 'fonttools',
            'sources': ['src/fonttools.cc'],
            'conditions': [
                [
                    'OS=="mac"',
                    {
                        'link_settings': {
                            'libraries': [
                                '/System/Library/Frameworks/Python.framework',
                                # '/usr/local/Cellar/python/2.7.13/Frameworks/Python.framework',
                            ]
                        },
                        'xcode_settings': {
                            'GCC_ENABLE_CPP_EXCEPTIONS': 'YES',
                            'MACOSX_DEPLOYMENT_TARGET': '10.7',
                            'WARNING_CFLAGS': [
                                '-Wno-unused-variable',
                                '-Wint-conversions',
                                '-Wmissing-field-initializers',
                                '-Wno-c++11-extensions'
                            ]
                        }
                    }
                ]
            ]
        }
    ]
}

cmake_minimum_required(VERSION 3.16)

project(therook VERSION 0.1 LANGUAGES CXX)

set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.2 COMPONENTS Core Quick Svg REQUIRED)

get_target_property(QMAKE Qt::qmake IMPORTED_LOCATION)

qt_add_executable(app_therook
        main.cpp
        )

add_subdirectory(disboard)
add_subdirectory(ui)

set_target_properties(app_therook PROPERTIES
        MACOSX_BUNDLE_GUI_IDENTIFIER my.example.com
        MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
        MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
        MACOSX_BUNDLE TRUE
        WIN32_EXECUTABLE TRUE
        )

target_link_libraries(app_therook
        PRIVATE Qt6::Quick
        disboardplugin uiplugin
        )

install(TARGETS app_therook
        BUNDLE DESTINATION .
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})

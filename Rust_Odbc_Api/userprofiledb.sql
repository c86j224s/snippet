USE [master]
GO
/****** Object:  Database [UserProfileDb]    Script Date: 2021-08-31 오후 8:54:54 ******/
CREATE DATABASE [UserProfileDb]
 CONTAINMENT = NONE
 ON  PRIMARY 
( NAME = N'UserProfileDb', FILENAME = N'/var/opt/mssql/data/UserProfileDb.mdf' , SIZE = 8192KB , MAXSIZE = UNLIMITED, FILEGROWTH = 65536KB )
 LOG ON 
( NAME = N'UserProfileDb_log', FILENAME = N'/var/opt/mssql/data/UserProfileDb_log.ldf' , SIZE = 8192KB , MAXSIZE = 2048GB , FILEGROWTH = 65536KB )
 WITH CATALOG_COLLATION = DATABASE_DEFAULT
GO
ALTER DATABASE [UserProfileDb] SET COMPATIBILITY_LEVEL = 150
GO
IF (1 = FULLTEXTSERVICEPROPERTY('IsFullTextInstalled'))
begin
EXEC [UserProfileDb].[dbo].[sp_fulltext_database] @action = 'enable'
end
GO
ALTER DATABASE [UserProfileDb] SET ANSI_NULL_DEFAULT OFF 
GO
ALTER DATABASE [UserProfileDb] SET ANSI_NULLS OFF 
GO
ALTER DATABASE [UserProfileDb] SET ANSI_PADDING OFF 
GO
ALTER DATABASE [UserProfileDb] SET ANSI_WARNINGS OFF 
GO
ALTER DATABASE [UserProfileDb] SET ARITHABORT OFF 
GO
ALTER DATABASE [UserProfileDb] SET AUTO_CLOSE OFF 
GO
ALTER DATABASE [UserProfileDb] SET AUTO_SHRINK OFF 
GO
ALTER DATABASE [UserProfileDb] SET AUTO_UPDATE_STATISTICS ON 
GO
ALTER DATABASE [UserProfileDb] SET CURSOR_CLOSE_ON_COMMIT OFF 
GO
ALTER DATABASE [UserProfileDb] SET CURSOR_DEFAULT  GLOBAL 
GO
ALTER DATABASE [UserProfileDb] SET CONCAT_NULL_YIELDS_NULL OFF 
GO
ALTER DATABASE [UserProfileDb] SET NUMERIC_ROUNDABORT OFF 
GO
ALTER DATABASE [UserProfileDb] SET QUOTED_IDENTIFIER OFF 
GO
ALTER DATABASE [UserProfileDb] SET RECURSIVE_TRIGGERS OFF 
GO
ALTER DATABASE [UserProfileDb] SET  DISABLE_BROKER 
GO
ALTER DATABASE [UserProfileDb] SET AUTO_UPDATE_STATISTICS_ASYNC OFF 
GO
ALTER DATABASE [UserProfileDb] SET DATE_CORRELATION_OPTIMIZATION OFF 
GO
ALTER DATABASE [UserProfileDb] SET TRUSTWORTHY OFF 
GO
ALTER DATABASE [UserProfileDb] SET ALLOW_SNAPSHOT_ISOLATION OFF 
GO
ALTER DATABASE [UserProfileDb] SET PARAMETERIZATION SIMPLE 
GO
ALTER DATABASE [UserProfileDb] SET READ_COMMITTED_SNAPSHOT OFF 
GO
ALTER DATABASE [UserProfileDb] SET HONOR_BROKER_PRIORITY OFF 
GO
ALTER DATABASE [UserProfileDb] SET RECOVERY FULL 
GO
ALTER DATABASE [UserProfileDb] SET  MULTI_USER 
GO
ALTER DATABASE [UserProfileDb] SET PAGE_VERIFY CHECKSUM  
GO
ALTER DATABASE [UserProfileDb] SET DB_CHAINING OFF 
GO
ALTER DATABASE [UserProfileDb] SET FILESTREAM( NON_TRANSACTED_ACCESS = OFF ) 
GO
ALTER DATABASE [UserProfileDb] SET TARGET_RECOVERY_TIME = 60 SECONDS 
GO
ALTER DATABASE [UserProfileDb] SET DELAYED_DURABILITY = DISABLED 
GO
ALTER DATABASE [UserProfileDb] SET ACCELERATED_DATABASE_RECOVERY = OFF  
GO
EXEC sys.sp_db_vardecimal_storage_format N'UserProfileDb', N'ON'
GO
ALTER DATABASE [UserProfileDb] SET QUERY_STORE = OFF
GO
USE [UserProfileDb]
GO
/****** Object:  Table [dbo].[UserCountries]    Script Date: 2021-08-31 오후 8:54:54 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[UserCountries](
	[Id] [uniqueidentifier] NOT NULL,
	[CountryCode] [varchar](4) NOT NULL,
	[Registered] [datetimeoffset](3) NOT NULL,
PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[UserProfiles]    Script Date: 2021-08-31 오후 8:54:54 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[UserProfiles](
	[Id] [uniqueidentifier] NOT NULL,
	[GenderCode] [tinyint] NOT NULL,
	[RealName] [nvarchar](64) NOT NULL,
	[PhoneNumber] [varchar](32) NOT NULL,
	[PrivacyAgreed] [bit] NOT NULL,
	[Registered] [datetimeoffset](3) NOT NULL,
	[Modified] [datetimeoffset](3) NULL,
PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
SET ANSI_PADDING ON
GO
/****** Object:  Index [IX_UserCountries_CountryCode]    Script Date: 2021-08-31 오후 8:54:54 ******/
CREATE NONCLUSTERED INDEX [IX_UserCountries_CountryCode] ON [dbo].[UserCountries]
(
	[CountryCode] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, DROP_EXISTING = OFF, ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
GO
/****** Object:  StoredProcedure [dbo].[p_GetCountriesAndProfilesById]    Script Date: 2021-08-31 오후 8:54:54 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
create procedure [dbo].[p_GetCountriesAndProfilesById]
	@id uniqueidentifier
as
begin
	set nocount on

	select Id, CountryCode, Registered from dbo.UserCountries (nolock) where Id = @id

	select Id, GenderCode, RealName, PhoneNumber, PrivacyAgreed, Registered, Modified from dbo.UserProfiles (nolock) where id = @id
	
	return 0
end
GO
/****** Object:  StoredProcedure [dbo].[p_GetUserCountryByCountryCode]    Script Date: 2021-08-31 오후 8:54:54 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
create procedure [dbo].[p_GetUserCountryByCountryCode]
	@countryCode varchar(4)
as
begin
	set nocount on

	select Id, CountryCode, Registered from dbo.UserCountries (nolock) where CountryCode = @countryCode

	return 0
end
GO
/****** Object:  StoredProcedure [dbo].[p_GetUserCountryById]    Script Date: 2021-08-31 오후 8:54:54 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
create procedure [dbo].[p_GetUserCountryById]
	@id uniqueidentifier
as
begin
	set nocount on

	select Id, CountryCode, Registered from dbo.UserCountries (nolock) where Id = @id

	return 0
end
GO
/****** Object:  StoredProcedure [dbo].[p_InsertUserCountry]    Script Date: 2021-08-31 오후 8:54:54 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

CREATE procedure [dbo].[p_InsertUserCountry]
	@id uniqueidentifier,
	@countryCode varchar(4),
	@registered datetimeoffset(3)
as
begin
	set nocount on

	insert into dbo.UserCountries (Id, CountryCode, Registered)
	output inserted.Id, inserted.CountryCode, inserted.Registered
	values (@id, @countryCode, @registered)
	
	return @@error

end
GO
/****** Object:  StoredProcedure [dbo].[p_UpsertUserCountry]    Script Date: 2021-08-31 오후 8:54:54 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO

create procedure [dbo].[p_UpsertUserCountry]
	@id uniqueidentifier,
	@countryCode varchar(4),
	@registered datetimeoffset(3)
as
begin
	set nocount on

	merge dbo.UserCountries as dst
	using (select @id, @countryCode, @registered) as src (Id, CountryCode, Registered)
	on (src.Id =  dst.Id)
	when matched then
		update set CountryCode = src.CountryCode, Registered = src.Registered
	when not matched then
		insert (Id, CountryCode, Registered)
		values (src.Id, src.CountryCode, src.Registered);
	
	return @@error

end
GO
USE [master]
GO
ALTER DATABASE [UserProfileDb] SET  READ_WRITE 
GO

